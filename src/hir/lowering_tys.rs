use crate::{
    ast::{ASTType, ASTTypeKind},
    hir::{
        hir::TypeNodeId,
        lowering::HIRLowering,
        types::{HIRType, HIRTypeKind, Prim},
    },
};

impl HIRLowering {
    fn type_ids(&mut self, tys: Vec<ASTType>) -> Option<Vec<TypeNodeId>> {
        tys.into_iter()
            .map(|t| self.lower_type(t).map(|ty| ty.id))
            .collect()
    }

    fn lower_type(&mut self, ty: ASTType) -> Option<HIRType> {
        let span = ty.span.clone();
        let id = self.type_id();

        let kind = match ty.kind {
            ASTTypeKind::I8
            | ASTTypeKind::U8
            | ASTTypeKind::U16
            | ASTTypeKind::I16
            | ASTTypeKind::I32
            | ASTTypeKind::U32
            | ASTTypeKind::I64
            | ASTTypeKind::U64
            | ASTTypeKind::F32
            | ASTTypeKind::F64
            | ASTTypeKind::C8
            | ASTTypeKind::C16
            | ASTTypeKind::C32
            | ASTTypeKind::Bool
            | ASTTypeKind::Unit
            | ASTTypeKind::Type => {
                let primitive = Prim::new(ty.kind);
                HIRTypeKind::Primitive(primitive)
            }
            ASTTypeKind::Pointer(inner) => {
                let inner = self.lower_type(*inner)?;
                HIRTypeKind::Pointer(inner.id)
            }
            ASTTypeKind::Applied { callee, args } => {
                let hir_callee = self.lower_type(*callee)?;
                let hir_args = self.type_ids(args)?;

                HIRTypeKind::Applied {
                    callee: hir_callee.id,
                    args: hir_args,
                }
            }
            ASTTypeKind::FunctionPtr {
                type_params,
                return_type,
            } => {
                let params = self.type_ids(type_params)?;
                let ret_ty = self.lower_type(*return_type)?.id;
                HIRTypeKind::Function {
                    params,
                    return_ty: ret_ty,
                }
            }
            ASTTypeKind::Path(path) => HIRTypeKind::Path(path),
            ASTTypeKind::Array(ty, size) => {
                let ty_id = self.lower_type(*ty)?.id;
                let len_id = self.lower_expr(&*size)?.id;
                HIRTypeKind::Array {
                    ty: ty_id,
                    length: len_id,
                }
            }
            ASTTypeKind::None => HIRTypeKind::Primitive(Prim::Error),
            /*_ => {
                self.report("Unknown type node".to_string(), Some(span.clone()));
                HIRTypeKind::Primitive(Prim::Error)
            }*/
        };

        let hir_ty = HIRType::new(id, kind, span);
        self.hir_module.types.push(hir_ty.clone());
        Some(hir_ty)
    }
}
