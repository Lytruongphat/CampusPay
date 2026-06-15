#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, Symbol,
};

#[contract]
pub struct CampusPayContract;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin, // Thêm key để lưu trữ Admin
    Balance(Address),
    Merchant(Address),
}

#[contractimpl]
impl CampusPayContract {

    // Khởi tạo Admin (Chỉ gọi 1 lần duy nhất sau khi deploy)
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    // Đăng ký cửa hàng trong khuôn viên trường
    pub fn register_merchant(env: Env, merchant: Address) {
        // Chỉ Admin mới được phép đăng ký cửa hàng
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        env.storage()
            .persistent()
            .set(&DataKey::Merchant(merchant), &true);
    }

    // Nạp token cho sinh viên
    pub fn mint(env: Env, student: Address, amount: i128) {
        if amount <= 0 {
            panic!("Amount must be positive");
        }

        // Chỉ Admin mới được quyền in/nạp token
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let key = DataKey::Balance(student.clone());

        let balance: i128 = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&key, &(balance + amount));
    }

    // Thanh toán trong khuôn viên
    pub fn pay(
        env: Env,
        student: Address,
        merchant: Address,
        amount: i128,
    ) {
        if amount <= 0 {
            panic!("Amount must be positive");
        }

        student.require_auth();

        let is_merchant: bool = env
            .storage()
            .persistent()
            .get(&DataKey::Merchant(merchant.clone()))
            .unwrap_or(false);

        if !is_merchant {
            panic!("Merchant not authorized");
        }

        let student_key = DataKey::Balance(student.clone());

        let student_balance: i128 = env
            .storage()
            .persistent()
            .get(&student_key)
            .unwrap_or(0);

        if student_balance < amount {
            panic!("Insufficient balance");
        }

        let merchant_key = DataKey::Balance(merchant.clone());

        let merchant_balance: i128 = env
            .storage()
            .persistent()
            .get(&merchant_key)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&student_key, &(student_balance - amount));

        env.storage()
            .persistent()
            .set(&merchant_key, &(merchant_balance + amount));
    }

    // Xem số dư
    pub fn balance(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(user))
            .unwrap_or(0)
    }

    // Kiểm tra cửa hàng có được phép hay không
    pub fn is_merchant(env: Env, merchant: Address) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::Merchant(merchant))
            .unwrap_or(false)
    }
}