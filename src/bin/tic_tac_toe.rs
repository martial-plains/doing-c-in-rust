#![no_std]
#![no_main]

use core::{
    ffi::{c_int, c_uint},
    ptr::{self},
};

use libc::{c_char, fgets, fprintf, printf, rand, scanf, srand, sscanf, strlen, timegm};

static mut GAME_TABLE: [c_char; 9] = [0; 9];

use libc_stdhandle::{stderr, stdin};

#[no_mangle]
unsafe extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    srand(timegm(ptr::null_mut()) as c_uint);

    let mut l: i32 = 0;

    loop {
        let mut n = 0;

        for item in GAME_TABLE.as_mut_slice() {
            *item = b'*' as i8;
        }

        printf(c"***************************************\n".as_ptr());
        printf(c"*************TIC TAC TOE***************\n".as_ptr());
        printf(c"***************************************\n".as_ptr());
        printf(c"***********1. YOU vs COMPUTER *********\n".as_ptr());
        printf(c"***********2. YOU vs PLAYER ***********\n".as_ptr());
        printf(c"***********3.EXIT *********************\n".as_ptr());
        printf(c"Enter your choice : ".as_ptr());
        scanf(c"%d".as_ptr(), &mut n);

        match n {
            1 => {
                singlemode();
            }

            2 => {
                doublemode();
            }

            _ => {
                printf(c"THANK YOU and EXIT!".as_ptr());
            }
        }

        printf(c"Next game ? : ".as_ptr());
        printf(c"Enter 1 - YES and 0 - NO: \n".as_ptr());
        scanf(c"%d".as_ptr(), &mut l);

        if l != 1 {
            break;
        }
    }

    0
}

unsafe fn doublemode() {
    let mut m: c_int = 0;
    let mut e1: c_int = 0;
    let mut k = 0;
    let mut double_table_count: c_int = 0;

    for _ in 0..3 {
        for _ in 0..3 {
            printf(c"%c ".as_ptr(), GAME_TABLE[k] as c_int);
            k += 1;
        }

        printf(c"\n".as_ptr());
    }

    for _ in 1..10 {
        k = 0;

        printf(c"PLAYER1 - where would you like to place 'x' : ".as_ptr());
        scanf(c"%d".as_ptr(), &mut m);

        placex(m);

        if double_table_count < 4 {
            printf(c"PLAYER2 - where would you like to place 'o' : ".as_ptr());
            scanf(c"%d".as_ptr(), &mut e1);
            placey(e1);
        }

        for _ in 0..3 {
            for _ in 0..3 {
                printf(c"%c ".as_ptr(), GAME_TABLE[k] as c_int);
                k += 1;
            }

            printf(c"\n".as_ptr());
        }

        double_table_count += 1;
        let o: c_int = checkwin();

        if o == -1 || o == -2 {
            if o == -1 {
                printf(c"Player 1 WIN\n".as_ptr());
            }

            if o == -2 {
                printf(c"Player 2 WIN\n".as_ptr());
            }

            break;
        }

        if double_table_count == 4 {
            printf(c"\nDRAW\n".as_ptr());
            break;
        }
    }
}

unsafe fn checkwin() -> c_int {
    if GAME_TABLE[0] == GAME_TABLE[1] && GAME_TABLE[1] == GAME_TABLE[2] {
        if GAME_TABLE[0] == b'x' as c_char
            && GAME_TABLE[1] == b'x' as c_char
            && GAME_TABLE[2] == b'x' as c_char
        {
            return -1;
        }

        if GAME_TABLE[0] == b'o' as c_char
            && GAME_TABLE[1] == b'o' as c_char
            && GAME_TABLE[2] == b'o' as c_char
        {
            return -2;
        }
    } else if GAME_TABLE[0] == GAME_TABLE[4] && GAME_TABLE[4] == GAME_TABLE[8] {
        if GAME_TABLE[0] == b'x' as c_char
            && GAME_TABLE[4] == b'x' as c_char
            && GAME_TABLE[8] == b'x' as c_char
        {
            return -1;
        }

        if GAME_TABLE[0] == b'o' as c_char
            && GAME_TABLE[4] == b'o' as c_char
            && GAME_TABLE[8] == b'o' as c_char
        {
            return -2;
        }
    } else if GAME_TABLE[0] == GAME_TABLE[3] && GAME_TABLE[3] == GAME_TABLE[6] {
        if GAME_TABLE[0] == b'x' as c_char
            && GAME_TABLE[3] == b'x' as c_char
            && GAME_TABLE[6] == b'x' as c_char
        {
            return -1;
        }

        if GAME_TABLE[0] == b'o' as c_char
            && GAME_TABLE[3] == b'o' as c_char
            && GAME_TABLE[6] == b'o' as c_char
        {
            return -2;
        }
    } else if GAME_TABLE[3] == GAME_TABLE[4] && GAME_TABLE[4] == GAME_TABLE[5] {
        if GAME_TABLE[3] == b'x' as c_char
            && GAME_TABLE[4] == b'x' as c_char
            && GAME_TABLE[5] == b'x' as c_char
        {
            return -1;
        }

        if GAME_TABLE[3] == b'o' as c_char
            && GAME_TABLE[4] == b'o' as c_char
            && GAME_TABLE[5] == b'o' as c_char
        {
            return -2;
        }
    } else if GAME_TABLE[6] == GAME_TABLE[7] && GAME_TABLE[7] == GAME_TABLE[8] {
        if GAME_TABLE[6] == b'x' as c_char
            && GAME_TABLE[7] == b'x' as c_char
            && GAME_TABLE[8] == b'x' as c_char
        {
            return -1;
        }

        if GAME_TABLE[6] == b'o' as c_char
            && GAME_TABLE[7] == b'o' as c_char
            && GAME_TABLE[8] == b'o' as c_char
        {
            return -2;
        }
    } else if GAME_TABLE[1] == GAME_TABLE[4] && GAME_TABLE[4] == GAME_TABLE[7] {
        if GAME_TABLE[1] == b'x' as c_char
            && GAME_TABLE[4] == b'x' as c_char
            && GAME_TABLE[7] == b'x' as c_char
        {
            return -1;
        }

        if GAME_TABLE[1] == b'o' as c_char
            && GAME_TABLE[4] == b'o' as c_char
            && GAME_TABLE[7] == b'o' as c_char
        {
            return -2;
        }
    } else if GAME_TABLE[2] == GAME_TABLE[5] && GAME_TABLE[5] == GAME_TABLE[8] {
        if GAME_TABLE[2] == b'x' as c_char
            && GAME_TABLE[5] == b'x' as c_char
            && GAME_TABLE[8] == b'x' as c_char
        {
            return -1;
        }

        if GAME_TABLE[2] == b'o' as c_char
            && GAME_TABLE[5] == b'o' as c_char
            && GAME_TABLE[8] == b'o' as c_char
        {
            return -2;
        }
    } else if GAME_TABLE[2] == GAME_TABLE[4] && GAME_TABLE[4] == GAME_TABLE[6] {
        if GAME_TABLE[2] == b'x' as c_char
            && GAME_TABLE[4] == b'x' as c_char
            && GAME_TABLE[6] == b'x' as c_char
        {
            return -1;
        }

        if GAME_TABLE[2] == b'o' as c_char
            && GAME_TABLE[4] == b'o' as c_char
            && GAME_TABLE[6] == b'o' as c_char
        {
            return -2;
        }
    }

    0
}

unsafe fn placey(e1: c_int) {
    let e1 = e1 as usize;

    if (1..=9).contains(&e1) {
        if GAME_TABLE[e1 - 1] != b'x' as c_char && GAME_TABLE[e1 - 1] != b'o' as c_char {
            GAME_TABLE[e1 - 1] = b'o' as c_char;
        } else {
            let n = check_placex();
            placex(n);
        }
    } else {
        let n = check_placex();
        placex(n);
    }
}

unsafe fn placex(m: c_int) {
    if (1..=9).contains(&m) {
        if GAME_TABLE[(m - 1) as usize] != b'x' as c_char
            && GAME_TABLE[(m - 1) as usize] != b'o' as c_char
        {
            GAME_TABLE[(m - 1) as usize] = b'x' as c_char;
        } else {
            let n = check_placex();
            placex(n);
        }
    }
}

unsafe fn singlemode() {
    let mut m = 0;
    let mut k = 0;
    let mut table_fill_count = 0;

    for _ in 0..3 {
        for _ in 0..3 {
            printf(c"%c ".as_ptr(), GAME_TABLE[k] as c_int);
            k += 1;
        }

        printf(c"\n".as_ptr());
    }

    for _ in 1..10 {
        k = 0;

        printf(c"Where would you like to place 'x' : ".as_ptr());
        scanf(c"%d".as_ptr(), &mut m);

        placex(m);
        if table_fill_count < 4 {
            place();
        }

        for _ in 0..3 {
            for _ in 0..3 {
                printf(c"%c ".as_ptr(), GAME_TABLE[k] as c_int);
                k += 1;
            }

            printf(c"\n".as_ptr());
        }

        table_fill_count += 1;
        let o = checkwin();

        if o == -1 || o == -2 {
            if o == -1 {
                printf(c"YOU WIN\n".as_ptr());
            }

            if o == -2 {
                printf(c"YOU LOSE\n".as_ptr());
            }

            break;
        }

        if table_fill_count == 4 {
            printf(c"\nDRAW\n".as_ptr());
            break;
        }
    }
}

unsafe fn place() {
    let e = rand() % 9;

    if e >= 0 {
        let e = e as usize;
        if GAME_TABLE[e] != b'x' as c_char && GAME_TABLE[e] != b'o' as c_char {
            GAME_TABLE[e as usize] = b'o' as c_char;
            printf(c"\n Computer placed at %d position\n".as_ptr(), e + 1);
        } else {
            place()
        }
    }
}

unsafe fn check_placex() -> c_int {
    let mut input = [b' ' as c_char; 50];
    let mut n1 = 0;

    loop {
        fgets(input.as_mut_ptr(), 49, stdin());

        if strlen(input.as_ptr()) > 2 || strlen(input.as_ptr()) == 0 {
            fprintf(stderr(), c"Invalid move, Enter number 1 - 9: ".as_ptr());
            continue;
        }

        if sscanf(input.as_ptr(), c"%d".as_ptr(), &mut n1) != 1 {
            fprintf(stderr(), c"Invalid move, Enter number 1 - 9: ".as_ptr());
            continue;
        }

        if (GAME_TABLE[n1 - 1] == b'x' as c_char)
            || (GAME_TABLE[n1 - 1]) == b'o' as c_char
            || (n1 == 0)
        {
            fprintf(stderr(), c"Already allocated, Enter number: ".as_ptr());
            continue;
        }
        return n1 as c_int;
    }
}
