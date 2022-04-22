use regex::Regex;

// interface Node {
//   type: string;
//   loc: SourceLocation | null;
// }
pub trait Node {}

// interface SourceLocation {
//   source: string | null;
//   start: Position;
//   end: Position;
// }
pub struct SourceLocation {
    source: Option<String>,
    start: Position,
    end: Position,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize) -> Self {
        SourceLocation {
            source: None,
            start: Position::new(line, column),
            end: Position::new(0, 0),
        }
    }
}

// interface Position {
//   line: number; // >= 1
//   column: number; // >= 0
// }
pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }
}

// interface Identifier <: Expression, Pattern {
//   type: "Identifier";
//   name: string;
// }
pub struct Identifier {
    pub loc: SourceLocation,
    pub name: String,
}

impl Identifier {
    pub fn new(name: String, start: Position, end: Position) -> Self {
        Identifier {
            loc: SourceLocation {
                source: Some(name.clone()),
                start,
                end,
            },
            name,
        }
    }
}

impl Node for Identifier {}

impl Expression for Identifier {}

impl Pattern for Identifier {}

// interface Literal <: Expression {
//   type: "Literal";
//   value: string | boolean | null | number | RegExp;
// }
pub enum LiteralValue {
    String(String),
    Boolean(bool),
    Null(Null),
    Number(f64),
    Bigint(i128),
    RegExp(Regex),
}

pub struct Literal {
    loc: SourceLocation,
    value: LiteralValue,
}

impl Node for Literal {}

impl Expression for Literal {}

// interface Program <: Node {
//   type: "Program";
//   body: [ Directive | Statement ];
// }
pub struct Program {
    pub loc: SourceLocation,
    pub body: Vec<Box<dyn Statement>>,
}

impl Node for Program {}

impl Program {
    pub fn new(line: usize, column: usize) -> Self {
        Program {
            loc: SourceLocation::new(line, column),
            body: vec![],
        }
    }
}

// interface Function <: Node {
//   id: Identifier | null;
//   params: [ Pattern ];
//   body: FunctionBody;
// }
pub trait Function {}

// interface Statement <: Node { }
pub trait Statement {}

pub struct Null;

// interface ExpressionStatement <: Statement {
//   type: "ExpressionStatement";
//   expression: Expression;
// }
pub struct ExpressionStatement {
    loc: SourceLocation,
    expression: Box<dyn Expression>,
    directive: Option<String>,
}

impl Node for ExpressionStatement {}

impl Statement for ExpressionStatement {}

// interface Directive <: ExpressionStatement {
//   expression: Literal;
//   directive: string;
// }
pub struct Directive {
    loc: SourceLocation,
    expression: Literal,
    directive: String,
}

impl Node for Directive {}

impl Statement for Directive {}

// interface BlockStatement <: Statement {
//   type: "BlockStatement";
//   body: [ Statement ];
// }
pub struct BlockStatement {
    loc: SourceLocation,
    body: Vec<Box<dyn Statement>>,
}

impl Node for BlockStatement {}

impl Statement for BlockStatement {}

// interface FunctionBody <: BlockStatement {
//   body: [ Directive | Statement ];
// }
pub struct FunctionBody {
    body: Vec<Box<dyn Statement>>,
}

impl FunctionBody {
    pub fn new() -> Self {
        FunctionBody { body: vec![] }
    }
}

impl Node for FunctionBody {}

impl Statement for FunctionBody {}

// interface EmptyStatement <: Statement {
//   type: "EmptyStatement";
// }
pub struct EmptyStatement;

impl Node for EmptyStatement {}

impl Statement for EmptyStatement {}

// interface DebuggerStatement <: Statement {
//   type: "DebuggerStatement";
// }
pub struct DebuggerStatement;

impl Node for DebuggerStatement {}

impl Statement for DebuggerStatement {}

// interface WithStatement <: Statement {
//   type: "WithStatement";
//   object: Expression;
//   body: Statement;
// }
pub struct WithStatement {
    object: Box<dyn Expression>,
    body: Box<dyn Statement>,
}

impl Node for WithStatement {}

impl Statement for WithStatement {}

// interface ReturnStatement <: Statement {
//   type: "ReturnStatement";
//   argument: Expression | null;
// }
pub struct ReturnStatement {
    argument: Option<Box<dyn Expression>>,
}

impl Node for ReturnStatement {}

impl Statement for ReturnStatement {}

// interface LabeledStatement <: Statement {
//   type: "LabeledStatement";
//   label: Identifier;
//   body: Statement;
// }
pub struct LabeledStatement {
    label: Identifier,
    body: Box<dyn Statement>,
}

impl Node for LabeledStatement {}

impl Statement for LabeledStatement {}

// interface BreakStatement <: Statement {
//   type: "BreakStatement";
//   label: Identifier | null;
// }
pub struct BreakStatement {
    label: Option<Identifier>,
}

impl Node for BreakStatement {}

impl Statement for BreakStatement {}

// interface ContinueStatement <: Statement {
//   type: "ContinueStatement";
//   label: Identifier | null;
// }
pub struct ContinueStatement {
    label: Option<Identifier>,
}

impl Node for ContinueStatement {}

impl Statement for ContinueStatement {}

// interface IfStatement <: Statement {
//   type: "IfStatement";
//   test: Expression;
//   consequent: Statement;
//   alternate: Statement | null;
// }
pub struct IfStatement {
    test: Box<dyn Expression>,
    consequent: Box<dyn Statement>,
    alternate: Option<Box<dyn Statement>>,
}

impl Node for IfStatement {}

impl Statement for IfStatement {}

// interface SwitchStatement <: Statement {
//   type: "SwitchStatement";
//   discriminant: Expression;
//   cases: [ SwitchCase ];
// }
pub struct SwitchStatement {
    discriminant: Box<dyn Expression>,
    cases: Vec<SwitchCase>,
}

impl Node for SwitchStatement {}

impl Statement for SwitchStatement {}

// interface SwitchCase <: Node {
//   type: "SwitchCase";
//   test: Expression | null;
//   consequent: [ Statement ];
// }
pub struct SwitchCase {
    test: Option<Box<dyn Expression>>,
    consequent: Vec<Box<dyn Statement>>,
}

impl Node for SwitchCase {}

// interface ThrowStatement <: Statement {
//   type: "ThrowStatement";
//   argument: Expression;
// }
pub struct ThrowStatement {
    argument: Box<dyn Expression>,
}

impl Node for ThrowStatement {}

impl Statement for ThrowStatement {}

// interface TryStatement <: Statement {
//   type: "TryStatement";
//   block: BlockStatement;
//   handler: CatchClause | null;
//   finalizer: BlockStatement | null;
// }
pub struct TryStatement {
    block: BlockStatement,
    handler: Option<CatchClause>,
    finalizer: Option<BlockStatement>,
}

impl Node for TryStatement {}

impl Statement for TryStatement {}

// interface CatchClause <: Node {
//   type: "CatchClause";
//   param: Pattern;
//   body: BlockStatement;
// }
pub struct CatchClause {
    param: Box<dyn Pattern>,
    body: BlockStatement,
}

impl Node for CatchClause {}

// interface WhileStatement <: Statement {
//   type: "WhileStatement";
//   test: Expression;
//   body: Statement;
// }
pub struct WhileStatement {
    test: Box<dyn Expression>,
    body: Box<dyn Statement>,
}

impl Node for WhileStatement {}

impl Statement for WhileStatement {}

// interface DoWhileStatement <: Statement {
//   type: "DoWhileStatement";
//   body: Statement;
//   test: Expression;
// }
pub struct DoWhileStatement {
    test: Box<dyn Expression>,
    body: Box<dyn Statement>,
}

impl Node for DoWhileStatement {}

impl Statement for DoWhileStatement {}

// interface ForStatement <: Statement {
//   type: "ForStatement";
//   init: VariableDeclaration | Expression | null;
//   test: Expression | null;
//   update: Expression | null;
//   body: Statement;
// }
pub enum ForStatementInit {
    VariableDeclaration(VariableDeclaration),
    Expression(Box<dyn Expression>),
}

pub struct ForStatement {
    init: Option<ForStatementInit>,
    test: Option<Box<dyn Expression>>,
    update: Option<Box<dyn Expression>>,
    body: Box<dyn Statement>,
}

impl Node for ForStatement {}

impl Statement for ForStatement {}

// interface ForInStatement <: Statement {
//   type: "ForInStatement";
//   left: VariableDeclaration |  Pattern;
//   right: Expression;
//   body: Statement;
// }
pub enum ForInStatementLeft {
    VariableDeclaration(VariableDeclaration),
    Expression(Box<dyn Expression>),
}

pub struct ForInStatement {
    left: ForInStatementLeft,
    right: Box<dyn Expression>,
    body: Box<dyn Statement>,
}

impl Node for ForInStatement {}

impl Statement for ForInStatement {}

// interface Declaration <: Statement { }
trait Declaration {}

// interface FunctionDeclaration <: Function, Declaration {
//   type: "FunctionDeclaration";
//   id: Identifier;
// }
pub struct FunctionDeclaration {
    id: Identifier,
    params: Vec<Box<dyn Pattern>>,
    body: FunctionBody,
}

impl FunctionDeclaration {
    pub fn new(id: Identifier) -> Self {
        FunctionDeclaration {
            id,
            params: vec![],
            body: FunctionBody::new(),
        }
    }
}

impl Node for FunctionDeclaration {}

impl Function for FunctionDeclaration {}

impl Statement for FunctionDeclaration {}

impl Declaration for FunctionDeclaration {}

// interface VariableDeclaration <: Declaration {
//   type: "VariableDeclaration";
//   declarations: [ VariableDeclarator ];
//   kind: "var";
// }
pub struct VariableDeclaration {
    declarations: Vec<VariableDeclarator>,
    kind: String,
}

impl Node for VariableDeclaration {}

impl Statement for VariableDeclaration {}

impl Declaration for VariableDeclaration {}

// interface VariableDeclarator <: Node {
//   type: "VariableDeclarator";
//   id: Pattern;
//   init: Expression | null;
// }
pub struct VariableDeclarator {
    id: Box<dyn Pattern>,
    init: Option<Box<dyn Expression>>,
}

impl Node for VariableDeclarator {}

// interface Expression <: Node { }
pub trait Expression {}

// interface ThisExpression <: Expression {
//   type: "ThisExpression";
// }
pub struct ThisExpression;

impl Node for ThisExpression {}

impl Expression for ThisExpression {}

// interface ArrayExpression <: Expression {
//   type: "ArrayExpression";
//   elements: [ Expression | null ];
// }
pub struct ArrayExpression {
    elements: Vec<Option<Box<dyn Expression>>>,
}

impl Node for ArrayExpression {}

impl Expression for ArrayExpression {}

// interface ObjectExpression <: Expression {
//   type: "ObjectExpression";
//   properties: [ Property ];
// }
pub struct ObjectExpression {
    properties: Vec<Property>,
}

impl Node for ObjectExpression {}

impl Expression for ObjectExpression {}

// interface Property <: Node {
//   type: "Property";
//   key: Literal | Identifier;
//   value: Expression;
//   kind: "init" | "get" | "set";
// }
pub enum PropertyKey {
    Literal(Literal),
    Identifier(Identifier),
}

pub enum PropertyKind {
    Init,
    Get,
    Set,
}

pub struct Property {
    key: PropertyKey,
    value: Box<dyn Expression>,
    kind: PropertyKind,
}

impl Node for Property {}

// interface FunctionExpression <: Function, Expression {
//   type: "FunctionExpression";
// }
pub struct FunctionExpression {
    params: Vec<Box<dyn Pattern>>,
    body: FunctionBody,
}

impl FunctionExpression {
    pub fn new() -> Self {
        FunctionExpression {
            params: vec![],
            body: FunctionBody::new(),
        }
    }
}

impl Node for FunctionExpression {}

impl Function for FunctionExpression {}

impl Expression for FunctionExpression {}

// interface UnaryExpression <: Expression {
//   type: "UnaryExpression";
//   operator: UnaryOperator;
//   prefix: boolean;
//   argument: Expression;
// }
pub enum UnaryOperator {
    Positive,
    Negative,
    LogicalInversion,
    BitwiseInversion,
    Typeof,
    Void,
    Delete,
}

pub struct UnaryExpression {
    operator: UnaryOperator,
    prefix: bool,
    argument: Box<dyn Expression>,
}

impl Node for UnaryExpression {}

impl Expression for UnaryExpression {}

// interface UpdateExpression <: Expression {
//   type: "UpdateExpression";
//   operator: UpdateOperator;
//   argument: Expression;
//   prefix: boolean;
// }
pub enum UpdateOperator {
    Increment,
    Decrement,
}

pub struct UpdateExpression {
    operator: UpdateOperator,
    prefix: bool,
    argument: Box<dyn Expression>,
}

impl Node for UpdateExpression {}

impl Expression for UpdateExpression {}

// interface BinaryExpression <: Expression {
//   type: "BinaryExpression";
//   operator: BinaryOperator;
//   left: Expression;
//   right: Expression;
// }
pub enum BinaryOperator {
    DoubleE,
    DoubleNE,
    TripleE,
    TripleNE,
    LT,
    LTE,
    GT,
    GTE,
    LeftShift,
    RightShift,
    URightShift,
    Plus,
    Minus,
    Multiple,
    Divide,
    Modulo,
    BitwiseOR,
    BitwiseXOR,
    BitwiseAND,
    In,
    Instanceof,
}

pub struct BinaryExpression {
    operator: BinaryOperator,
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Node for BinaryExpression {}

impl Expression for BinaryExpression {}

// interface AssignmentExpression <: Expression {
//   type: "AssignmentExpression";
//   operator: AssignmentOperator;
//   left: Pattern | Expression;
//   right: Expression;
// }
pub enum AssignmentOperator {
    Normal,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    NullishCoalescing,
}

pub enum AssignmentExpressionLeft {
    Pattern(Box<dyn Pattern>),
    Expression(Box<dyn Expression>),
}

pub struct AssignmentExpression {
    operator: AssignmentOperator,
    left: AssignmentExpressionLeft,
    right: Box<dyn Expression>,
}

impl Node for AssignmentExpression {}

impl Expression for AssignmentExpression {}

// interface LogicalExpression <: Expression {
//   type: "LogicalExpression";
//   operator: LogicalOperator;
//   left: Expression;
//   right: Expression;
// }
pub enum LogicalOperator {
    LogicalOR,
    LogicalAND,
}

pub struct LogicalExpression {
    operator: AssignmentOperator,
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Node for LogicalExpression {}

impl Expression for LogicalExpression {}

// interface MemberExpression <: Expression, Pattern {
//   type: "MemberExpression";
//   object: Expression;
//   property: Expression;
//   computed: boolean;
// }
pub struct MemberExpression {
    operator: AssignmentOperator,
    object: Box<dyn Expression>,
    property: Box<dyn Expression>,
    computed: bool,
}

impl Node for MemberExpression {}

impl Pattern for MemberExpression {}

impl Expression for MemberExpression {}

// interface ConditionalExpression <: Expression {
//   type: "ConditionalExpression";
//   test: Expression;
//   alternate: Expression;
//   consequent: Expression;
// }
pub struct ConditionalExpression {
    test: Box<dyn Expression>,
    alternate: Box<dyn Expression>,
    consequent: Box<dyn Expression>,
}

impl Node for ConditionalExpression {}

impl Expression for ConditionalExpression {}

// interface CallExpression <: Expression {
//   type: "CallExpression";
//   callee: Expression;
//   arguments: [ Expression ];
// }
pub struct CallExpression {
    callee: Box<dyn Expression>,
    arguments: Vec<Box<dyn Expression>>,
}

impl Node for CallExpression {}

impl Expression for CallExpression {}

// interface NewExpression <: Expression {
//   type: "NewExpression";
//   callee: Expression;
//   arguments: [ Expression ];
// }
pub struct NewExpression {
    callee: Box<dyn Expression>,
    arguments: Vec<Box<dyn Expression>>,
}

impl Node for NewExpression {}

impl Expression for NewExpression {}

// interface SequenceExpression <: Expression {
//   type: "SequenceExpression";
//   expressions: [ Expression ];
// }
pub struct SequenceExpression {
    expressions: Vec<Box<dyn Expression>>,
}

impl Node for SequenceExpression {}

impl Expression for SequenceExpression {}

// interface Pattern <: Node { }
pub trait Pattern {}
