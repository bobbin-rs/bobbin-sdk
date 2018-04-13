// #[macro_export]
// macro_rules! tree {
//     { 
//         id: $id:ident: $ty:ident, 
//         $(clock: { 
//           $($cid:ident : $cty:ident,)* 
//         },)*
//         $(type: { 
//             $($ppty:path : $pcty:ident,)* 
//         },)*
//     } => {
//         pub const $id: $ty = $ty {};
//         #[derive(Debug, Default, Clone, Copy)]
//         pub struct $ty {}

//         // Clock ID Definitions
//         $(
//             $(
//                 pub const $cid: $cty = $cty {};
//                 #[derive(Debug, Default, Clone, Copy)]
//                 pub struct $cty {}
//                 impl $crate::Id for $cty {}   
//             )*
//         )*

//         // Peripheral Clock ID Definitions
//         $(
//             $(
//                 impl $crate::IdFor<$ppty> for $ty { type Out = $pcty; }
//             )*
//         )*
//     }
// }

// #[macro_export]
// macro_rules! tree_impl {
//     { 
//         id: $tree_id:ident, 
//         defn: $defn_id:ident : $defn_ty:ident, 
//         $(clock_impl: {
//             $($clk_ty:ident: $clk_impl_ty:ident { $clk_impl_expr:expr },)*
//         })*
//     } => {
//         // pub const $tree_id: $crate::Tree<$defn_ty, TreeImpl> = $crate::Tree::new($defn_id, TREE_IMPL);
//         pub const $tree_id: Tree = Tree {};

//         #[derive(Default, Debug, Clone, Copy)]
//         pub struct Tree {}

//         impl $crate::ClockTree for Tree {
//             type Defn = $defn_ty;
//             type Impl = TreeImpl;
//         }

//         // pub const TREE_IMPL: TreeImpl = TreeImpl {};
//         #[derive(Default, Debug)]
//         pub struct TreeImpl {}

//         // Clock Impl Definitions
//         $(
//             $(                            
//                 impl $crate::ClockForId<$clk_ty> for TreeImpl { type Out = $clk_impl_ty; }

//                 #[derive(Debug, Default, Clone, Copy)]
//                 pub struct $clk_impl_ty {}
//                 impl $crate::Clock for $clk_impl_ty { fn read() -> $crate::Hz { $clk_impl_expr.into() } }            
//             )*
//         )*
//     }
// }