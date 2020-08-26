use swc_ecma_ast::ImportDecl;
use swc_ecma_visit::{noop_visit_mut_type, FoldWith, VisitMut, VisitMutWith};

// use swc_ecmascript::ast::ImportDecl;
// struct SVGImports {
//     name: Option<Box<Expr>>,
// }

// impl Fold for SVGImports {
//     noop_fold_type!();

//     /// Don't recurse into array.
//     fn fold_array_lit(&mut self, node: ArrayLit) -> ArrayLit {
//         node
//     }

//     fn fold_call_expr(&mut self, expr: CallExpr) -> CallExpr {
//         let expr = expr.fold_children_with(self);

//         if is_create_class_call(&expr) {
//             let name = match self.name.take() {
//                 Some(name) => name,
//                 None => return expr,
//             };
//             add_display_name(expr, name)
//         } else {
//             expr
//         }
//     }
//     /// Don't recurse into object.
//     fn fold_object_lit(&mut self, node: ObjectLit) -> ObjectLit {
//         node
//     }
// }

pub struct SVGImportToComponent;

impl VisitMut for SVGImportToComponent {
    noop_visit_mut_type!();

    fn visit_mut_import_decl(&mut self, decl: &mut ImportDecl) {
        println!("{:#?}", decl);
    }
}
