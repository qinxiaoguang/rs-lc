pub mod l002;
pub mod l003;
pub mod l004;
pub mod l005;
pub mod l006;
pub mod l007;
pub mod l008;
pub mod l009;
pub mod l010;
pub mod l011;
pub mod l014;
pub mod l015;
pub mod l016;
pub mod l017;
pub mod l018;
pub mod l019;
pub mod l020;
pub mod l021;
pub mod l022;
pub mod l023;
pub mod l024;
pub mod l025;
pub mod l026;
pub mod l027;
pub mod l029;
pub mod l030;
pub mod l031;
pub mod l032;
pub mod l033;
pub mod l034;
pub mod l035;
pub mod l036;
pub mod l038;
pub mod l039;
pub mod l040;
pub mod l041;
pub mod l042;
pub mod l043;
pub mod l045;
pub mod l046;
pub mod l047;
pub mod l048;
pub mod l049;
pub mod l050;
pub mod l051;
pub mod l052;
pub mod l053;
pub mod l054;
pub mod l055;
pub mod l056;
pub mod l057;
pub mod l058;
pub mod l059;
pub mod l060;
pub mod l061;
pub mod l071;
pub mod l072;
pub mod l073;
pub mod l074;
pub mod l075;
pub mod l076;
pub mod l077;
pub mod l078;
pub mod l079;
pub mod l080;
pub mod l081;
pub mod l082;
pub mod l083;
pub mod l084;
pub mod l085;
pub mod l086;
pub mod l088;
pub mod l089;
pub mod l090;
pub mod l091;
pub mod l092;
pub mod l093;
pub mod l094;
pub mod l095;
pub mod l096;
pub mod l097;
pub mod l098;
pub mod l105;
pub mod l106;
pub mod l109;
pub mod l118;
pub mod l119;
pub mod l120;
pub mod l121;
pub mod l122;
pub mod l123;
pub mod l143;
pub mod l148;
pub mod l203;
pub mod l206;
pub mod l234;
pub mod l445;
pub mod l725;
pub mod l766;
pub mod util;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn to_list(vec: Vec<i32>) -> Option<Box<Self>> {
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
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
}
