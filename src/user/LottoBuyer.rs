use std::borrow::BorrowMut;
use std::cell::RefCell;
use crate::LottoPaper;

pub struct LottoBuyer {
    // amount: RefCell<i32>,
    amount: i32,
    // lottoBox: RefCell<Vec<LottoPaper>>,
    lottoBox: Vec<LottoPaper>,
}

impl LottoBuyer {
    pub fn new() -> LottoBuyer {
        LottoBuyer {
            amount: 0,
            // lottoBox: RefCell::new(Vec::new()),
            lottoBox: Vec::new(),
        }
    }

    pub fn add_lotto(&mut self, lottoPaper: LottoPaper) {
        // self.lottoBox.borrow_mut().push(lottoPaper);
        self.lottoBox.push(lottoPaper);
    }

    pub fn plus_amount(&mut self, money: i32) -> i32 {
        self.amount += money;
        self.amount
    }

    pub fn minus_amount(&mut self, &money: i64) -> i32 {
        self.amount -= money;
        self.amount
    }
}