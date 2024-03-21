/**
* This is not an instruction, this is a module that contains all the instructions
*/
pub mod adc;
pub mod and;
pub mod asl;
pub mod bcc;
pub mod bcs;
pub mod beq;
pub mod bit;
pub mod bmi;
pub mod bne;
pub mod bpl;
pub mod brk;
pub mod bvc;
pub mod bvs;
pub mod clc;
pub mod cld;
pub mod cli;
pub mod clv;
pub mod cmp;
pub mod consts;
pub mod cpx;
pub mod cpy;
pub mod dec;
pub mod dex;
pub mod dey;
pub mod eor;
pub mod inc;
pub mod inx;
pub mod iny;
pub mod jmp;
pub mod jsr;
pub mod lda;
pub mod ldx;
pub mod ldy;
pub mod nop;
pub mod ora;
pub mod pha;
pub mod php;
pub mod pla;
pub mod plp;
pub mod rol;
pub mod ror;
pub mod rti;
pub mod rts;
pub mod sbc;
pub mod sec;
pub mod sed;
pub mod sei;
pub mod sta;
pub mod stx;
pub mod sty;

pub use adc::adc;
pub use and::and;
pub use asl::asl;
pub use bcc::bcc;
pub use bcs::bcs;
pub use beq::beq;
pub use bit::bit;
pub use bmi::bmi;
pub use bne::bne;
pub use bpl::bpl;
pub use brk::brk;
pub use bvc::bvc;
pub use bvs::bvs;
pub use clc::clc;
pub use cld::cld;
pub use cli::cli;
pub use clv::clv;
pub use cmp::cmp;
pub use consts::{CPU_CLOCK_SPEED, IRQ_VECTOR, NMI_VECTOR, RESET_VECTOR};
pub use cpx::cpx;
pub use cpy::cpy;
pub use dec::dec;
pub use dex::dex;
pub use dey::dey;
pub use eor::eor;
pub use inc::inc;
pub use inx::inx;
pub use iny::iny;
pub use jmp::jmp;
pub use jsr::jsr;
pub use lda::lda;
pub use ldx::ldx;
pub use ldy::ldy;
pub use nop::nop;
pub use ora::ora;
pub use pha::pha;
pub use php::php;
pub use pla::pla;
pub use plp::plp;
pub use rol::rol;
pub use ror::ror;
pub use rti::rti;
pub use rts::rts;
pub use sbc::sbc;
pub use sec::sec;
pub use sed::sed;
pub use sei::sei;
pub use sta::sta;
pub use stx::stx;
pub use sty::sty;
