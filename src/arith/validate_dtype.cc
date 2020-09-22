#include <tvm/tir/expr.h>
#include <tvm/tir/expr_functor.h>

namespace tvm {
namespace arith {

class ValidateDataType: public tir::ExprVisitor {
    void VisitExpr(const PrimExpr& expr) {
        CHECK_NE(expr->dtype->bits, 0);
        tir::ExprVisitor::VisitExpr(expr);
    }
};

void ValidateDType(const PrimExpr& expr) {
    ValidateDataType()(expr);
}

} // namespace arith
} // namespace tvm

//   // list of functions to override.
//   void VisitExpr_(const VarNode* op) override;
//   void VisitExpr_(const SizeVarNode* op) override;
//   void VisitExpr_(const LoadNode* op) override;
//   void VisitExpr_(const BufferLoadNode* op) override;
//   void VisitExpr_(const ProducerLoadNode* op) override;
//   void VisitExpr_(const LetNode* op) override;
//   void VisitExpr_(const CallNode* op) override;
//   void VisitExpr_(const AddNode* op) override;
//   void VisitExpr_(const SubNode* op) override;
//   void VisitExpr_(const MulNode* op) override;
//   void VisitExpr_(const DivNode* op) override;
//   void VisitExpr_(const ModNode* op) override;
//   void VisitExpr_(const FloorDivNode* op) override;
//   void VisitExpr_(const FloorModNode* op) override;
//   void VisitExpr_(const MinNode* op) override;
//   void VisitExpr_(const MaxNode* op) override;
//   void VisitExpr_(const EQNode* op) override;
//   void VisitExpr_(const NENode* op) override;
//   void VisitExpr_(const LTNode* op) override;
//   void VisitExpr_(const LENode* op) override;
//   void VisitExpr_(const GTNode* op) override;
//   void VisitExpr_(const GENode* op) override;
//   void VisitExpr_(const AndNode* op) override;
//   void VisitExpr_(const OrNode* op) override;
//   void VisitExpr_(const ReduceNode* op) override;
//   void VisitExpr_(const CastNode* op) override;
//   void VisitExpr_(const NotNode* op) override;
//   void VisitExpr_(const SelectNode* op) override;
//   void VisitExpr_(const RampNode* op) override;
//   void VisitExpr_(const BroadcastNode* op) override;
//   void VisitExpr_(const ShuffleNode* op) override;
//   void VisitExpr_(const IntImmNode* op) override;
//   void VisitExpr_(const FloatImmNode* op) override;
//   void VisitExpr_(const StringImmNode* op) override;
//   void VisitExpr_(const AnyNode* op) override;
// };
