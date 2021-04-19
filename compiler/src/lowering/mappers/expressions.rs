use fe_analyzer::Context;
use fe_parser::ast as fe;
use fe_parser::node::Node;

/// Lowers an expression and all sub expressions.
pub fn expr(context: &Context, exp: Node<fe::Expr>) -> Node<fe::Expr> {
    let lowered_kind = match exp.kind {
        fe::Expr::Name(_) => exp.kind,
        fe::Expr::Num(_) => exp.kind,
        fe::Expr::Bool(_) => exp.kind,
        fe::Expr::Subscript { value, slices } => fe::Expr::Subscript {
            value: boxed_expr(context, value),
            slices: slices_index_expr(context, slices),
        },
        fe::Expr::Attribute { value, attr } => fe::Expr::Attribute {
            value: boxed_expr(context, value),
            attr,
        },
        fe::Expr::Ternary {
            if_expr,
            test,
            else_expr,
        } => fe::Expr::Ternary {
            if_expr: boxed_expr(context, if_expr),
            test: boxed_expr(context, test),
            else_expr: boxed_expr(context, else_expr),
        },
        fe::Expr::BoolOperation { left, op, right } => fe::Expr::BoolOperation {
            left: boxed_expr(context, left),
            op,
            right: boxed_expr(context, right),
        },
        fe::Expr::BinOperation { left, op, right } => fe::Expr::BinOperation {
            left: boxed_expr(context, left),
            op,
            right: boxed_expr(context, right),
        },
        fe::Expr::UnaryOperation { op, operand } => fe::Expr::UnaryOperation {
            op,
            operand: boxed_expr(context, operand),
        },
        fe::Expr::CompOperation { left, op, right } => fe::Expr::CompOperation {
            left: boxed_expr(context, left),
            op,
            right: boxed_expr(context, right),
        },
        fe::Expr::Call { func, args } => fe::Expr::Call {
            func: boxed_expr(context, func),
            args: call_args(context, args),
        },
        fe::Expr::List { .. } => unimplemented!(),
        fe::Expr::ListComp { .. } => unimplemented!(),
        // We only accept empty tuples for now. We may want to completely eliminate tuple
        // expressions before the Yul codegen pass, tho.
        fe::Expr::Tuple { .. } => exp.kind,
        fe::Expr::Str(_) => exp.kind,
        fe::Expr::Ellipsis => unimplemented!(),
    };

    Node::new(lowered_kind, exp.span)
}

fn slices_index_expr(
    context: &Context,
    slices: Node<Vec<Node<fe::Slice>>>,
) -> Node<Vec<Node<fe::Slice>>> {
    let first_slice = &slices.kind[0];

    if let fe::Slice::Index(exp) = &first_slice.kind {
        return Node::new(
            vec![Node::new(
                fe::Slice::Index(Box::new(expr(context, *exp.to_owned()))),
                first_slice.span,
            )],
            slices.span,
        );
    }

    unreachable!()
}

/// Lowers and optional expression.
pub fn optional_expr(context: &Context, exp: Option<Node<fe::Expr>>) -> Option<Node<fe::Expr>> {
    exp.map(|exp| expr(context, exp))
}

/// Lowers a boxed expression.
#[allow(clippy::boxed_local)]
pub fn boxed_expr(context: &Context, exp: Box<Node<fe::Expr>>) -> Box<Node<fe::Expr>> {
    Box::new(expr(context, *exp))
}

/// Lowers a list of expression.
pub fn multiple_exprs(context: &Context, exp: Vec<Node<fe::Expr>>) -> Vec<Node<fe::Expr>> {
    exp.into_iter().map(|exp| expr(context, exp)).collect()
}

fn call_args(
    context: &Context,
    args: Node<Vec<Node<fe::CallArg>>>,
) -> Node<Vec<Node<fe::CallArg>>> {
    let lowered_args = args
        .kind
        .into_iter()
        .map(|arg| match arg.kind {
            fe::CallArg::Arg(inner_arg) => {
                Node::new(fe::CallArg::Arg(expr(context, inner_arg)), arg.span)
            }
            fe::CallArg::Kwarg(inner_arg) => {
                Node::new(fe::CallArg::Kwarg(kwarg(context, inner_arg)), arg.span)
            }
        })
        .collect();

    Node::new(lowered_args, args.span)
}

fn kwarg(context: &Context, kwarg: fe::Kwarg) -> fe::Kwarg {
    fe::Kwarg {
        name: kwarg.name,
        value: boxed_expr(context, kwarg.value),
    }
}