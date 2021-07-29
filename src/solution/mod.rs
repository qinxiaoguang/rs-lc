#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[allow(dead_code)]
    pub fn to_list(vec: Vec<i32>) -> Option<Box<Self>> {
        let mut head = Self::new(0);
        let mut tail = &mut head;
        for val in vec.into_iter() {
            tail.next = Some(Box::new(Self::new(val)));
            tail = tail.next.as_mut().unwrap();
        }
        head.next
    }
}

// 二叉树
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn add_left(&mut self, node: Self) -> &mut Self {
        self.left = Some(Rc::new(RefCell::new(node)));
        self
    }

    pub fn add_right(&mut self, node: Self) -> &mut Self {
        self.right = Some(Rc::new(RefCell::new(node)));
        self
    }

    pub fn to_tree(v: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        Self::get_root(v, 0)
    }

    fn get_root(v: &[i32], root_node: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if root_node >= v.len() {
            return None;
        }
        if v[root_node] == -1 {
            return None;
        }
        let mut root = TreeNode::new(v[root_node]);
        root.left = Self::get_root(v, root_node * 2 + 1);
        root.right = Self::get_root(v, root_node * 2 + 2);
        Some(Rc::new(RefCell::new(root)))
    }
}

pub struct Solution {}

// vec![String::from(),String::from()...]
macro_rules! sv{
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push(String::from($x));
            )*
            v
        }
    }
}

// String
macro_rules! s {
    ($x:expr) => {
        String::from($x)
    };
}

// vec![vec![],vec![]...]
macro_rules! matrix{
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                let mut item = Vec::new();
                for &i in $x.iter(){
                    item.push(i);
                }
                v.push(item);
            )*
            v
        }
    }
}

mod a000;
mod j003;
mod j004;
mod j005;
mod j006;
mod j007;
mod j009;
mod j010;
mod j010v2;
mod j011;
mod j012;
mod j013;
mod j016;
mod j017;
mod j018;
mod j019;
mod j020;
mod j021;
mod j022;
mod j026;
mod j027;
mod j028;
mod j029;
mod j030;
mod j031;
mod j032;
mod j032v2;
mod j032v3;
mod j033;
mod j034;
mod j038;
mod j039;
mod j040;
mod j041;
mod j042;
mod j043;
mod j044;
mod j045;
mod j046;
mod j047;
mod j048;
mod j050;
mod j051;
mod j053;
mod j053v2;
mod j054;
mod j055v1;
mod j055v2;
mod j056v1;
mod j056v2;
mod j057;
mod j057v2;
mod j058v1;
mod j058v2;
mod j059v1;
mod j059v2;
mod j060;
mod j061;
mod j062;
mod j063;
mod j064;
mod j065;
mod j066;
mod l002;
mod l003;
mod l004;
mod l005;
mod l006;
mod l007;
mod l008;
mod l009;
mod l010;
mod l011;
mod l014;
mod l015;
mod l016;
mod l017;
mod l018;
mod l019;
mod l020;
mod l021;
mod l022;
mod l023;
mod l024;
mod l025;
mod l026;
mod l027;
mod l029;
mod l030;
mod l031;
mod l032;
mod l033;
mod l034;
mod l035;
mod l036;
mod l037;
mod l038;
mod l039;
mod l040;
mod l041;
mod l042;
mod l043;
mod l045;
mod l046;
mod l047;
mod l048;
mod l049;
mod l050;
mod l051;
mod l052;
mod l053;
mod l054;
mod l055;
mod l056;
mod l057;
mod l058;
mod l059;
mod l060;
mod l061;
mod l062;
mod l063;
mod l064;
mod l066;
mod l069;
mod l070;
mod l071;
mod l072;
mod l073;
mod l074;
mod l075;
mod l076;
mod l077;
mod l078;
mod l079;
mod l080;
mod l081;
mod l082;
mod l083;
mod l084;
mod l085;
mod l086;
mod l088;
mod l089;
mod l090;
mod l091;
mod l092;
mod l093;
mod l094;
mod l095;
mod l096;
mod l097;
mod l098;
mod l1002;
mod l1009;
mod l1013;
mod l1016;
mod l1020;
mod l1027;
mod l1031;
mod l105;
mod l1052;
mod l106;
mod l1072;
mod l1079;
mod l1080;
mod l109;
mod l1131;
mod l118;
mod l119;
mod l120;
mod l121;
mod l122;
mod l123;
mod l126;
mod l128;
mod l1296;
mod l1297;
mod l1405;
mod l1419;
mod l143;
mod l1465;
mod l1477;
mod l148;
mod l1487;
mod l1494;
mod l152;
mod l153;
mod l154;
mod l1594;
mod l162;
mod l1662;
mod l167;
mod l1679;
mod l169;
mod l171;
mod l1796;
mod l1802;
mod l1877;
mod l189;
mod l203;
mod l206;
mod l209;
mod l216;
mod l217;
mod l219;
mod l228;
mod l229;
mod l234;
mod l236;
mod l238;
mod l260;
mod l268;
mod l283;
mod l287;
mod l289;
mod l337;
mod l344;
mod l380;
mod l388;
mod l414;
mod l421;
mod l424;
mod l438;
mod l442;
mod l445;
mod l448;
mod l457;
mod l462;
mod l464;
mod l468;
mod l485;
mod l495;
mod l507;
mod l523;
mod l532;
mod l542;
mod l560;
mod l561;
mod l565;
mod l566;
mod l581;
mod l605;
mod l611;
mod l621;
mod l628;
mod l636;
mod l643;
mod l657;
mod l661;
mod l665;
mod l667;
mod l670;
mod l674;
mod l689;
mod l695;
mod l697;
mod l713;
mod l714;
mod l717;
mod l718;
mod l719;
mod l724;
mod l725;
mod l729;
mod l747;
mod l748;
mod l766;
mod l773;
mod l777;
mod l779;
mod l780;
mod l790;
mod l797;
mod l798;
mod l808;
mod l811;
mod l828;
mod l849;
mod l852;
mod l857;
mod l861;
mod l864;
mod l870;
mod l879;
mod l883;
mod l898;
mod l903;
mod l905;
mod l924;
mod l927;
mod l931;
mod l932;
mod l936;
mod l941;
mod l943;
mod l950;
mod l951;
mod l954;
mod l957;
mod l958;
mod l962;
mod l974;
mod l978;
mod l982;
mod l989;
mod l991;
mod m0405;
mod m0802;
mod m1005;
mod util;
