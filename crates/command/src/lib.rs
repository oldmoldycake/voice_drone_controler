#[derive(Debug, PartialEq)]
pub enum Direction {
    Forward,
    Back,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq)]
pub enum Unit {
    In,
    Ft,
    Y,
    Mi,
    Mm,
    Cm,
    M,
    Km,
}
#[derive(Debug, PartialEq)]
pub enum Command {
    Takeoff {
        altitude: f32,
        unit: Unit,
    },
    Land,
    Move {
        direction: Direction,
        distance: f32,
        unit: Unit,
    },
    Rotate {
        direction: Direction,
        degrees: f32,
    },
    Hover,
    ReturnHome,
}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    Empty,
    MissingValue,
    UnknownAction,
    UnknownDirection,
    MissingDirection,
    MissingDistance,
    InvalidDistance,
    MissingUnit,
    UnknownUnit,
}
fn resolve_unit(unit_str: Option<&str>) -> Result<Unit, CommandError> {
    let unit = match unit_str {
        //Imperial
        Some("in") => Unit::In,
        Some("ft") => Unit::Ft,
        Some("y") => Unit::Y,
        Some("mi") => Unit::Mi,
        //Metric
        Some("mm") => Unit::Mm,
        Some("cm") => Unit::Cm,
        Some("m") => Unit::M,
        Some("km") => Unit::Km,
        //Other
        Some(_) => return Err(CommandError::MissingUnit),
        None => return Err(CommandError::MissingUnit),
    };
    Ok(unit)
}
pub fn parse_command_string(input: &str) -> Result<Command, CommandError> {
    let mut parts = input.split_whitespace().peekable();

    let action = parts.next().ok_or(CommandError::Empty)?;

    match action {
        "mv" => {
            let command_string_direction = match parts.peek().copied() {
                None => return Err(CommandError::MissingDirection),
                Some(tok) if tok.parse::<f32>().is_ok() => {
                    return Err(CommandError::MissingDirection);
                }
                Some(tok) => {
                    parts.next();

                    match tok {
                        "f" => Direction::Forward,
                        "b" => Direction::Back,
                        "l" => Direction::Left,
                        "r" => Direction::Right,
                        "u" => Direction::Up,
                        "d" => Direction::Down,
                        _ => return Err(CommandError::UnknownDirection),
                    }
                }
            };
            let command_string_distance = parts
                .next()
                .ok_or(CommandError::MissingDistance)?
                .parse::<f32>()
                .map_err(|_| CommandError::InvalidDistance)?;

            let command_string_unit = resolve_unit(parts.next())?;

            Ok(Command::Move {
                direction: command_string_direction,
                distance: command_string_distance,
                unit: command_string_unit,
            })
        }
        "yaw" => {
            let command_string_direction = match parts.peek().copied() {
                None => return Err(CommandError::MissingDirection),
                Some(tok) if tok.parse::<f32>().is_ok() => {
                    return Err(CommandError::MissingDirection);
                }
                Some(tok) if !matches!(tok, "l" | "r") => {
                    return Err(CommandError::UnknownDirection);
                }
                Some(tok) => {
                    parts.next();

                    match tok {
                        "l" => Direction::Left,
                        "r" => Direction::Right,
                        _ => return Err(CommandError::UnknownDirection),
                    }
                }
            };

            let command_string_degrees = parts
                .next()
                .ok_or(CommandError::MissingDistance)?
                .parse::<f32>()
                .map_err(|_| CommandError::InvalidDistance)?;

            Ok(Command::Rotate {
                direction: command_string_direction,
                degrees: command_string_degrees,
            })
        }
        "alt" => todo!(),
        "takeoff" => todo!(),
        "land" => Ok(Command::Land),
        "rtl" => todo!(),
        "hold" => todo!(),
        _ => Err(CommandError::UnknownAction),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Testing moving
    #[test]
    fn test_empty_command_string() {
        assert_eq!(parse_command_string(""), Err(CommandError::Empty))
    }
    //Directions
    #[test]
    fn test_mv_forward_command() {
        assert_eq!(
            parse_command_string("mv f 10 ft"),
            Ok(Command::Move {
                direction: Direction::Forward,
                distance: 10.0,
                unit: Unit::Ft
            })
        );
    }

    #[test]
    fn test_mv_backwards_command() {
        assert_eq!(
            parse_command_string("mv b 10 ft"),
            Ok(Command::Move {
                direction: Direction::Back,
                distance: 10.0,
                unit: Unit::Ft
            })
        )
    }

    #[test]
    fn test_mv_left_command() {
        assert_eq!(
            parse_command_string("mv l 10 ft"),
            Ok(Command::Move {
                direction: Direction::Left,
                distance: 10.0,
                unit: Unit::Ft
            })
        )
    }

    #[test]
    fn test_mv_right_command() {
        assert_eq!(
            parse_command_string("mv r 10 ft"),
            Ok(Command::Move {
                direction: Direction::Right,
                distance: 10.0,
                unit: Unit::Ft
            })
        )
    }

    #[test]
    fn test_mv_up_command() {
        assert_eq!(
            parse_command_string("mv u 10 ft"),
            Ok(Command::Move {
                direction: Direction::Up,
                distance: 10.0,
                unit: Unit::Ft
            })
        )
    }

    #[test]
    fn test_mv_down_command() {
        assert_eq!(
            parse_command_string("mv d 10 ft"),
            Ok(Command::Move {
                direction: Direction::Down,
                distance: 10.0,
                unit: Unit::Ft
            })
        )
    }

    #[test]
    fn test_invalid_direction() {
        assert_eq!(
            parse_command_string("mv sw 10 ft"),
            Err(CommandError::UnknownDirection)
        )
    }

    #[test]
    fn test_missing_direction() {
        assert_eq!(
            parse_command_string("mv 10 ft"),
            Err(CommandError::MissingDirection)
        )
    }
    //Units
    #[test]
    fn test_ft_unit() {
        assert_eq!(
            parse_command_string("mv f 10 ft"),
            Ok(Command::Move {
                direction: Direction::Forward,
                distance: 10.0,
                unit: Unit::Ft
            })
        )
    }

    #[test]
    fn test_in_unit() {
        assert_eq!(
            parse_command_string("mv f 10 in"),
            Ok(Command::Move {
                direction: Direction::Forward,
                distance: 10.0,
                unit: Unit::In
            })
        )
    }

    #[test]
    fn test_y_unit() {
        assert_eq!(
            parse_command_string("mv f 10 y"),
            Ok(Command::Move {
                direction: Direction::Forward,
                distance: 10.0,
                unit: Unit::Y
            })
        )
    }

    #[test]
    fn test_mi_unit() {
        assert_eq!(
            parse_command_string("mv f 10 mi"),
            Ok(Command::Move {
                direction: Direction::Forward,
                distance: 10.0,
                unit: Unit::Mi
            })
        )
    }

    #[test]
    fn test_mm_unit() {
        assert_eq!(
            parse_command_string("mv f 10 mm"),
            Ok(Command::Move {
                direction: Direction::Forward,
                distance: 10.0,
                unit: Unit::Mm
            })
        )
    }

    #[test]
    fn test_cm_unit() {
        assert_eq!(
            parse_command_string("mv f 10 cm"),
            Ok(Command::Move {
                direction: Direction::Forward,
                distance: 10.0,
                unit: Unit::Cm
            })
        )
    }

    #[test]
    fn test_m_unit() {
        assert_eq!(
            parse_command_string("mv f 10 m"),
            Ok(Command::Move {
                direction: Direction::Forward,
                distance: 10.0,
                unit: Unit::M
            })
        )
    }

    #[test]
    fn test_km_unit() {
        assert_eq!(
            parse_command_string("mv f 10 km"),
            Ok(Command::Move {
                direction: Direction::Forward,
                distance: 10.0,
                unit: Unit::Km
            })
        )
    }

    //Testing rotation
    #[test]
    fn test_rotate_left() {
        assert_eq!(
            parse_command_string("yaw l 45 deg"),
            Ok(Command::Rotate {
                direction: Direction::Left,
                degrees: 45.0
            })
        )
    }

    #[test]
    fn test_rotate_right() {
        assert_eq!(
            parse_command_string("yaw r 45 deg"),
            Ok(Command::Rotate {
                direction: Direction::Right,
                degrees: 45.0
            })
        )
    }

    #[test]
    fn test_invalid_rotate_direction() {
        assert_eq!(
            parse_command_string("yaw u 45 deg"),
            Err(CommandError::UnknownDirection)
        )
    }
}
