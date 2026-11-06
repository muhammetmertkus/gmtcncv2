//! Stable panel button bit masks shared by the panel and its visualizations.

pub const PROGRAM_PROTECT: u64 = 1 << 0;
pub const ESTOP: u64 = 1 << 7;
pub const CYCLE_START: u64 = 1 << 14;
pub const CYCLE_STOP: u64 = 1 << 15;
pub const RAPID: u64 = 1 << 63;
pub const MLK: u64 = 1 << 55;
pub const FLUSH: u64 = 1 << 47;
pub const SPD_SP: u64 = 1 << 39;
pub const K2: u64 = 1 << 31;
pub const JOG_XN: u64 = 1 << 62;
pub const BDT: u64 = 1 << 54;
pub const SPD_CTS: u64 = 1 << 46;
pub const SPD_CW: u64 = 1 << 38;
pub const ARM_MAN: u64 = 1 << 30;
pub const JOG_YP: u64 = 1 << 61;
pub const SBK: u64 = 1 << 53;
pub const COOLANT: u64 = 1 << 45;
pub const MAG_CCW: u64 = 1 << 37;
pub const POT_DN: u64 = 1 << 29;
pub const JOG_ZP: u64 = 1 << 60;
pub const MPG_SIM: u64 = 1 << 52;
pub const PT_LA: u64 = 1 << 44;
pub const MAG_CW: u64 = 1 << 36;
pub const CHUCK: u64 = 1 << 28;
pub const JOG_AN: u64 = 1 << 59;
pub const JOG_AP: u64 = 1 << 51;
pub const AIR_BLOW: u64 = 1 << 43;
pub const SPD_ORI: u64 = 1 << 35;
pub const CHIP_CCW: u64 = 1 << 27;
pub const JOG_BP: u64 = 1 << 58;
pub const JOG_ZN: u64 = 1 << 50;
pub const WORKLAMP: u64 = 1 << 42;
/// Hardware-verified on the machine. This table must stay in step with
/// `linc-linuxcnc/src/panel.rs`: K1 sits on bit 34, K2 on bit 31 and LUB on
/// bit 32. K2 was on bit 34 in the first port, aliasing K1.
pub const K1: u64 = 1 << 34;
pub const CHIP_SP: u64 = 1 << 26;
pub const SAFE_DOOR: u64 = 1 << 57;
pub const JOG_YN: u64 = 1 << 49;
pub const DRN: u64 = 1 << 41;
pub const CORRIDOR: u64 = 1 << 33;
pub const CHIP_CW: u64 = 1 << 25;
pub const K4: u64 = 1 << 17;
pub const JOG_BN: u64 = 1 << 56;
pub const JOG_XP: u64 = 1 << 48;
pub const OPT: u64 = 1 << 40;
pub const LUB: u64 = 1 << 32;
pub const SPD_CCW: u64 = 1 << 24;
pub const K3: u64 = 1 << 16;
// Lathe-only keys. The lathe panel carries a second spindle (SPD.2.*), a
// spindle jog and a feed brake, a tailstock, one chip conveyor, one
// tool-change key and MST, none of which the mill panel has. The C-axis jog
// pair and C/S come from the same panel's jog group; the mill table has no
// C axis at all, so they need masks of their own.
pub const JOG_CN: u64 = 1 << 11;
pub const JOG_CP: u64 = 1 << 12;
pub const C_S: u64 = 1 << 18;
pub const SPD2_CW: u64 = 1 << 1;
pub const SPD2_SP: u64 = 1 << 2;
pub const SPD2_CCW: u64 = 1 << 3;
pub const SPD_JOG: u64 = 1 << 4;
pub const FEED_BRAKE: u64 = 1 << 5;
pub const TAIL: u64 = 1 << 6;
pub const CHIP: u64 = 1 << 8;
pub const TOOL_CHG: u64 = 1 << 9;
pub const MST: u64 = 1 << 10;

pub const BUTTON_PINS: [(&str, u64); 58] = [
    ("program_protect", self::PROGRAM_PROTECT),
    ("estop", self::ESTOP),
    ("cycle_start", self::CYCLE_START),
    ("cycle_stop", self::CYCLE_STOP),
    ("rapid", self::RAPID),
    ("mlk", self::MLK),
    ("flush", self::FLUSH),
    ("spd_sp", self::SPD_SP),
    ("k2", self::K2),
    ("jog_xn", self::JOG_XN),
    ("bdt", self::BDT),
    ("spd_cts", self::SPD_CTS),
    ("spd_cw", self::SPD_CW),
    ("arm_man", self::ARM_MAN),
    ("jog_yp", self::JOG_YP),
    ("sbk", self::SBK),
    ("coolant", self::COOLANT),
    ("mag_ccw", self::MAG_CCW),
    ("pot_dn", self::POT_DN),
    ("jog_zp", self::JOG_ZP),
    ("mpg_sim", self::MPG_SIM),
    ("pt_la", self::PT_LA),
    ("mag_cw", self::MAG_CW),
    ("chuck", self::CHUCK),
    ("jog_an", self::JOG_AN),
    ("jog_ap", self::JOG_AP),
    ("air_blow", self::AIR_BLOW),
    ("spd_ori", self::SPD_ORI),
    ("chip_ccw", self::CHIP_CCW),
    ("jog_bp", self::JOG_BP),
    ("jog_zn", self::JOG_ZN),
    ("worklamp", self::WORKLAMP),
    ("k1", self::K1),
    ("chip_sp", self::CHIP_SP),
    ("safe_door", self::SAFE_DOOR),
    ("jog_yn", self::JOG_YN),
    ("drn", self::DRN),
    ("corridor", self::CORRIDOR),
    ("chip_cw", self::CHIP_CW),
    ("k4", self::K4),
    ("jog_bn", self::JOG_BN),
    ("jog_xp", self::JOG_XP),
    ("opt", self::OPT),
    ("lube", self::LUB),
    ("spd_ccw", self::SPD_CCW),
    ("k3", self::K3),
    // Lathe-only keys, appended so the mill pin order stays stable.
    ("spd2_cw", self::SPD2_CW),
    ("spd2_sp", self::SPD2_SP),
    ("spd2_ccw", self::SPD2_CCW),
    ("spd_jog", self::SPD_JOG),
    ("feed_brake", self::FEED_BRAKE),
    ("tail", self::TAIL),
    ("chip", self::CHIP),
    ("tool_chg", self::TOOL_CHG),
    ("mst", self::MST),
    ("jog_cn", self::JOG_CN),
    ("jog_cp", self::JOG_CP),
    ("c_s", self::C_S),
];
