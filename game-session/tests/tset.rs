use game_session_io::*;
use gtest::{Log, ProgramBuilder, System};

const GAME_SESSION_PROGRAM_ID: u64 = 100;
const WORDLE_PROGRAM_ID: u64 = 200;

const USER: u64 = 64;

#[test]
fn test_start_game_without_initialization() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER, 600000000000000000);
    let game_session_program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
            .with_id(GAME_SESSION_PROGRAM_ID)
            .build(&system);

    // Attempt to start the game without initialization
    let _result = game_session_program.send(USER, GameSessionAction::StartGame);
}

#[test]
fn test_win() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER, 1_000_000_000_000_000_000);

    let game_session_program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
            .with_id(GAME_SESSION_PROGRAM_ID)
            .build(&system);
    let wordle_program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
            .with_id(WORDLE_PROGRAM_ID)
            .build(&system);

    let result = wordle_program.send_bytes(USER, []);
    println!("Wordle init result: {:?}", result);
    system.run_next_block();

    let result = game_session_program.send(
        USER,
        GameSessionInit {
            wordle_program_id: WORDLE_PROGRAM_ID.into(),
        },
    );
    system.run_next_block();

    println!("Game session init result: {:?}", result);

    let result = game_session_program.send(USER, GameSessionAction::StartGame);
    system.run_next_block();

    println!("Start game result: {:?}", result);

    let result = game_session_program.send(
        USER,
        GameSessionAction::CheckWord {
            word: "house".to_string(),
        },
    );
    system.run_next_block();

    println!("First word check result: {:?}", result);
    system.mint_to(USER, 1_000_000_000_000_000_000);

    let result = game_session_program.send(
        USER,
        GameSessionAction::CheckWord {
            word: "human".to_string(),
        },
    );
    system.run_next_block();

    println!("Second word check result: {:?}", result);
    system.mint_to(USER, 1_000_000_000_000_000_000);
}

#[test]
fn test_invalid_word_input() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER, 600000000000000000);

    let game_session_program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
            .with_id(GAME_SESSION_PROGRAM_ID)
            .build(&system);
    let wordle_program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
            .with_id(WORDLE_PROGRAM_ID)
            .build(&system);

    let _result = wordle_program.send_bytes(USER, []);
    game_session_program.send(
        USER,
        GameSessionInit {
            wordle_program_id: WORDLE_PROGRAM_ID.into(),
        },
    );

    // StartGame success
    game_session_program.send(USER, GameSessionAction::StartGame);

    // Try to send an invalid word (not 5 letters)
    let _result = game_session_program.send(
        USER,
        GameSessionAction::CheckWord {
            word: "invalid".to_string(), // Invalid word
        },
    );
}

#[test]
fn test_lose_exceeded_tries_limit() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER, 1_000_000_000_000_000_000);

    let game_session_program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
            .with_id(GAME_SESSION_PROGRAM_ID)
            .build(&system);
    let wordle_program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
            .with_id(WORDLE_PROGRAM_ID)
            .build(&system);

    let result = wordle_program.send_bytes(USER, []);
    println!("Wordle init result: {:?}", result);
    system.run_next_block();
    system.mint_to(USER, 1_000_000_000_000_000_000);

    let result = game_session_program.send(
        USER,
        GameSessionInit {
            wordle_program_id: WORDLE_PROGRAM_ID.into(),
        },
    );
    println!("Game session init result: {:?}", result);
    system.run_next_block();
    system.mint_to(USER, 1_000_000_000_000_000_000);

    let result = game_session_program.send(USER, GameSessionAction::StartGame);
    println!("Start game result: {:?}", result);
    system.run_next_block();
    system.mint_to(USER, 1_000_000_000_000_000_000);

    for i in 1..=5 {
        let result = game_session_program.send(
            USER,
            GameSessionAction::CheckWord {
                word: "wrong".to_string(),
            },
        );
        println!("Word check attempt {}: {:?}", i, result);
        system.run_next_block();
        system.mint_to(USER, 1_000_000_000_000_000_000);
    }
}

#[test]
fn test_lose_timeout() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER, 600000000000000000);
    let game_session_program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
            .with_id(GAME_SESSION_PROGRAM_ID)
            .build(&system);
    let wordle_program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
            .with_id(WORDLE_PROGRAM_ID)
            .build(&system);

    let _result = wordle_program.send_bytes(USER, []);
    system.run_next_block();

    let _result = game_session_program.send(
        USER,
        GameSessionInit {
            wordle_program_id: WORDLE_PROGRAM_ID.into(),
        },
    );
    system.run_next_block();

    let _result = game_session_program.send(USER, GameSessionAction::StartGame);
    system.run_next_block();
    let _log = Log::builder()
        .dest(USER)
        .source(GAME_SESSION_PROGRAM_ID)
        .payload(GameSessionEvent::StartSuccess);

    system.run_to_block(201);
    let _log = Log::builder()
        .dest(USER)
        .source(GAME_SESSION_PROGRAM_ID)
        .payload(GameSessionEvent::GameOver(GameStatus::Lose));
    println!("{:?}", _log);
}
