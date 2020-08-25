/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

use crate::runtime::{Object, ObjectPtr, String as TVMString};
use crate::DataType;

use super::*;

macro_rules! define_node {
    ($name:ident, $ref:expr, $typekey:expr; $node:ident { $($id:ident : $t:ty),*}) => {
        #[repr(C)]
        #[derive(Object)]
        #[ref_name = $ref]
        #[type_key = $typekey]
        pub struct $node {
            base: PrimExprNode,
            $(pub $id : $t),*
        }

        impl $name {
            pub fn new(datatype: DataType, $($id : $t,)*) -> $name {
                let base = PrimExprNode::base::<$node>(datatype);
                let node = $node { base, $($id),* };
                $name(Some(ObjectPtr::new(node)))
            }
        }

        impl From<$name> for PrimExpr {
            // TODO(@jroesch): Remove we with subtyping traits.
            fn from(x: $name) -> PrimExpr {
                x.downcast().expect(concat!(
                    "Failed to downcast `",
                    stringify!($name),
                    "` to PrimExpr"))
            }
        }
    }
}

define_node!(IntImm, "IntImm", "IntImm";
             IntImmNode { value: i64 });
define_node!(Var, "Var", "tir.Var";
             VarNode { name_hint: TVMString });

define_node!(Add, "Add", "tir.Add";
             AddNode { a: PrimExpr, b: PrimExpr });

define_node!(Sub, "Sub", "tir.Sub";
             SubNode { a: PrimExpr, b: PrimExpr });
define_node!(Mul, "Mul", "tir.Mul";
             MulNode { a: PrimExpr, b: PrimExpr });

define_node!(Div, "Div", "tir.Div";
             DivNode { a: PrimExpr, b: PrimExpr });
define_node!(Mod, "Mod", "tir.Mod";
             ModNode { a: PrimExpr, b: PrimExpr });
define_node!(FloorDiv, "FloorDiv", "tir.FloorDiv";
             FloorDivNode { a: PrimExpr, b: PrimExpr });
define_node!(FloorMod, "FloorMod", "tir.FloorMod";
             FloorModNode { a: PrimExpr, b: PrimExpr });

define_node!(Min, "Min", "tir.Min";
             MinNode { a: PrimExpr, b: PrimExpr });
define_node!(Max, "Max", "tir.Max";
             MaxNode { a: PrimExpr, b: PrimExpr });

// the new datatype is in the base expr
define_node!(Cast, "Cast", "tir.Cast";
             CastNode { value: PrimExpr });

// renamed base to start to avoid name clash
define_node!(Ramp, "Ramp", "tir.Ramp";
             RampNode { start: PrimExpr, stride: PrimExpr, lanes: i32 });
