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

    fn extract_string(&mut self, max_size: usize) -> Result<&'a str, ()> {
        // Stub for actual logic
        return Err(());
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

            players.extend([PlayerInfo {
                name: player_name,
                score: player_score,
                duration: TimeDelta::max_value(),
            }]);
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
                0.0,
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
}
