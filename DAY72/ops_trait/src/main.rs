// 연산자 오버로딩을 위한 트레이트
use std::ops::{
    /* 단항 연산자 */
    Neg,            //  -x  =  x.neg()
    Not,            //  !x  =  x.not()

    /* 산술 연산자 */
    Add,            //  x + y  =  x.add(y)
    Sub,            //  x - y  =  x.sub(y)
    Mul,            //  x * y  =  x.mul(y)
    Div,            //  x / y  =  x.div(y)
    Rem,            //  x % y  =  x.rem(y)

    /* 비트별 연산자 */
    BitAnd,           //  x &  y  =  x.bitand(y)
    BitOr,            //  x |  y  =  x.bitor(y)
    BitXor,           //  x ^  y  =  x.bitxor(y)
    Shl,              //  x << y  =  x.shl(y)
    Shr,              //  x >> y  =  x.shr(y)

    /* 산술 복합 배정 연산자 */
    AddAssign,                //  x += y  =  x.add_assign(y)
    SubAssign,                //  x -= y  =  x.sub_assign(y)
    MulAssign,                //  x *= y  =  x.mul_assign(y)
    DivAssign,                //  x /= y  =  x.div_assign(y)
    RemAssign,                //  x &= y  =  x.rem_assign(y)

    /* 비트별 복합 배정 연산자 */
    BitAndAssign,               //  x &=  y  =  x.bitand_assign(y)
    BitOrAssign,                //  x |=  y  =  x.bitor_assign(y)
    BitXorAssign,               //  x ^=  y  =  x.bitxor_assign(y)
    ShlAssign,                  //  x <<= y  =  x.shl_assign(y)
    ShrAssign,                  //  x >>= y  =  x.shr_assign(y)
};

fn main() {
    println!("Hello, world!");
}
