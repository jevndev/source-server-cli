use chrono::TimeDelta;

const PLAYER_INFO_RETURN_COMMAND: u8 = 0x44;
const PLAYER_NAME_MAX_SIZE: usize = 32;

pub struct PlayerInfo {
    name: String,
    score: u64,
    duration: chrono::TimeDelta,
}

pub struct PlayersInfo {
    players: Vec<PlayerInfo>,
}

struct ByteArrayWithExtraction<'a> {
    _bytes: &'a [u8],
    index: usize,
}

impl<'a> ByteArrayWithExtraction<'a> {
    pub fn new(_bytes: &'a [u8]) -> ByteArrayWithExtraction {
        return ByteArrayWithExtraction { _bytes, index: 0 };
    }

    // TODO: Refactor this to use generics once I figure out how rust generics work

    fn extract_u8(&mut self) -> Result<u8, ()> {
        if self.index >= self._bytes.len() {
            return Err(());
        }

        let value = self._bytes[self.index];
        self.index += 1;
        Ok(value)
    }

    fn extract_u64(&mut self) -> Result<u64, ()> {
        if self.index >= self._bytes.len() {
            return Err(());
        }

        let bytes: [u8; 8] = self._bytes[self.index..self.index + 8].try_into().unwrap();
        let value = u64::from_le_bytes(bytes);
        self.index += 8;
        return Ok(value);
    }

    fn extract_f32(&mut self) -> Result<f32, ()> {
        if self.index >= self._bytes.len() {
            return Err(());
        }

        let bytes: [u8; 4] = self._bytes[self.index..self.index + 4].try_into().unwrap();
        let value = f32::from_le_bytes(bytes);
        self.index += 4;
        return Ok(value);
    }

    fn extract_string(&mut self, max_size: usize) -> Result<String, ()> {
        let ending_index = self._bytes.len().min(max_size + self.index);
        for string_index in self.index..ending_index {
            if self._bytes[string_index] == 0x00 {
                let resultant_string_bytes = &self._bytes[self.index..string_index + 1];
                self.index = string_index + 1;
                return Ok(String::from_utf8_lossy(resultant_string_bytes).into_owned());
            }
        }

        return Err(());
    }

    fn exhausted(&self) -> bool {
        return self.index >= self._bytes.len();
    }
}

impl PlayersInfo {
    pub fn from_bytes(_bytes: &[u8]) -> Result<PlayersInfo, ()> {
        if _bytes.len() < 2 {
            return Err(());
        }

        let mut byte_array = ByteArrayWithExtraction::new(_bytes);

        let command = byte_array.extract_u8()?;
        if command != PLAYER_INFO_RETURN_COMMAND {
            return Err(());
        }

        let player_count = byte_array.extract_u8()?;

        if player_count != 0 && _bytes.len() == 2 {
            return Err(());
        }

        if player_count == 0 && _bytes.len() > 2 {
            return Err(());
        }

        let mut players: Vec<PlayerInfo> = vec![];

        for _ in 0..player_count {
            let _player_chunk_index = byte_array.extract_u8()?;
            let player_name = byte_array.extract_string(PLAYER_NAME_MAX_SIZE)?;
            let player_score = byte_array.extract_u64()?;
            let player_duration_seconds = byte_array.extract_f32()?;

            let player_duration = TimeDelta::new(player_duration_seconds as i64, 0);

            if player_duration.is_none() {
                return Err(());
            }

            players.extend([PlayerInfo {
                name: player_name,
                score: player_score,
                duration: player_duration.unwrap(),
            }]);
        }

        if !byte_array.exhausted() {
            return Err(());
        }

        return Ok(PlayersInfo { players });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_player_data_bytes(index: u8, name: &str, score: u64, duration: f32) -> Vec<u8> {
        let mut player_data = vec![index];
        player_data.extend(name.as_bytes());
        player_data.push(0x00);
        player_data.extend(score.to_le_bytes());
        player_data.extend(duration.to_le_bytes());
        return player_data;
    }

    fn make_message_from_players(players: &[PlayerInfo]) -> Vec<u8> {
        let num_players = players.len();
        assert!(num_players <= u8::MAX as usize);
        let num_players_byte = num_players as u8;
        let mut payload = vec![PLAYER_INFO_RETURN_COMMAND, num_players_byte];

        for player in players {
            payload.extend(make_player_data_bytes(
                0x00,
                &player.name,
                player.score,
                player.duration.num_seconds() as f32,
            ));
        }

        return payload;
    }

    #[test]
    fn from_bytes_should_return_error_when_no_bytes_provided() {
        let input_bytes: [u8; 0] = [];
        let result = PlayersInfo::from_bytes(&input_bytes);
        assert!(result.is_err())
    }

    #[test]
    fn from_bytes_should_return_error_when_header_has_invalid_command_flag() {
        let invalid_return_command_byte = PLAYER_INFO_RETURN_COMMAND + 1;
        let payload = [invalid_return_command_byte];
        let result = PlayersInfo::from_bytes(&payload);
        assert!(result.is_err());
    }

    #[test]
    fn from_bytes_should_return_error_when_missing_player_count() {
        let payload = [PLAYER_INFO_RETURN_COMMAND];
        let result = PlayersInfo::from_bytes(&payload);
        assert!(result.is_err());
    }

    #[test]
    fn from_bytes_should_return_empty_player_list_when_playercount_is_zero() {
        let payload = [PLAYER_INFO_RETURN_COMMAND, 0x00];
        let result = PlayersInfo::from_bytes(&payload);
        assert!(result.is_ok_and(|players_info| players_info.players.is_empty()));
    }

    #[test]
    fn from_bytes_should_return_error_when_remaining_bytes_after_player_count_chunks() {
        let mut payload: Vec<u8> = vec![PLAYER_INFO_RETURN_COMMAND, 0x01];
        let player_a_data = make_player_data_bytes(0, "foo", 10, 0.0);
        let player_b_data = make_player_data_bytes(1, "bar", 10, 0.0);

        payload.extend(player_a_data);
        payload.extend(player_b_data);

        let result = PlayersInfo::from_bytes(payload.as_slice());
        assert!(result.is_err());
    }

    #[test]
    fn from_bytes_should_return_all_players_in_payload() {
        let payload = make_message_from_players(&[
            PlayerInfo {
                name: "bar".to_string(),
                score: 10,
                duration: TimeDelta::new(10, 0).unwrap(),
            },
            PlayerInfo {
                name: "foo".to_string(),
                score: 10,
                duration: TimeDelta::new(10, 0).unwrap(),
            },
        ]);

        let result = PlayersInfo::from_bytes(payload.as_slice());

        assert!(result.is_ok());

        let player_info = result.unwrap();

        // TODO: make this actually check that the returned players are accurate
        assert!(player_info.players.len() == 2);
    }

    #[test]
    fn from_bytes_should_return_error_when_bytearray_size_doesnt_match_zero_playercount() {
        let payload = [PLAYER_INFO_RETURN_COMMAND, 0x00, 0x01];
        let result = PlayersInfo::from_bytes(&payload);
        assert!(result.is_err());
    }

    #[test]
    fn extract_string_from_bytes_should_return_error_for_empty_bytearray() {
        let _bytes: [u8; 0] = [];
        let mut byte_array = ByteArrayWithExtraction::new(&_bytes);
        const MAX_SIZE_GREATER_THAN_ZERO: usize = 1;
        let extracted_string = byte_array.extract_string(MAX_SIZE_GREATER_THAN_ZERO);
        assert!(extracted_string.is_err());
    }

    #[test]
    fn extract_string_from_bytes_should_return_empty_string_for_bytearray_of_only_null_terminator()
    {
        let _bytes: [u8; 1] = [0x00];
        let mut byte_array = ByteArrayWithExtraction::new(&_bytes);
        const MAX_SIZE_GREATER_THAN_1: usize = 2;
        let extracted_string = byte_array.extract_string(MAX_SIZE_GREATER_THAN_1);

        // assert that the string is okay and is only the null terminator
        assert!(extracted_string.is_ok());
        assert_eq!(extracted_string.unwrap(), "\0");
    }

    #[test]
    fn extract_string_from_bytes_should_return_error_if_no_string_found_in_bytearray() {
        const STRING_WITHOUT_NULL_TERMINATOR: [u8; 2] = ['a' as u8, 'b' as u8];
        const MAX_SIZE_LARGER_THAN_STRING_LENGTH: usize = STRING_WITHOUT_NULL_TERMINATOR.len() + 1;
        let mut byte_array = ByteArrayWithExtraction::new(&STRING_WITHOUT_NULL_TERMINATOR);
        let extracted_string = byte_array.extract_string(MAX_SIZE_LARGER_THAN_STRING_LENGTH);
        assert!(extracted_string.is_err());
    }

    #[test]
    fn extract_string_from_bytes_should_return_error_if_no_string_found_in_bytearray_before_max_size(
    ) {
        const STRING_WITHOUT_NULL_TERMINATOR: [u8; 2] = ['a' as u8, 'b' as u8];
        const MAX_SIZE_SMALLER_THAN_STRING_LENGTH: usize = STRING_WITHOUT_NULL_TERMINATOR.len() - 1;
        let mut byte_array = ByteArrayWithExtraction::new(&STRING_WITHOUT_NULL_TERMINATOR);
        let extracted_string = byte_array.extract_string(MAX_SIZE_SMALLER_THAN_STRING_LENGTH);
        assert!(extracted_string.is_err());
    }

    #[test]
    fn extract_string_from_bytes_should_return_same_string_as_input() {
        const INPUT_STRING: &str = "Hello World\0";
        let input_bytes = INPUT_STRING.as_bytes();

        let mut byte_array = ByteArrayWithExtraction::new(input_bytes);
        const MAX_SIZE_LONGER_THAN_STRING_LENGTH: usize = INPUT_STRING.len() + 1;
        let extracted_string = byte_array.extract_string(MAX_SIZE_LONGER_THAN_STRING_LENGTH);

        assert!(extracted_string.is_ok());

        let result_string = extracted_string.unwrap();

        assert_eq!(result_string, INPUT_STRING);
    }
}
