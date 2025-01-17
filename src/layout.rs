use crate::{
    Alignments, Colors, FlexBasis, FlexDirection, GridAuto, GridFlow, GridTemplate,
    MultiColumnLineStyle, Tree, Unit,
};

// Basic Layout
/// To be honest, `Container` is a part of Flex family, but he is too brilliant to stay in Flex family, Layout calls him.
pub struct Container {
    pub child: Tree,
    pub style: ContainerStyle,
}

/// `Container` style
#[derive(Default)]
pub struct ContainerStyle {
    pub align: Alignments,
    pub height: Unit,
    pub width: Unit,
    pub padding: Unit,
    pub margin: Unit,
    pub background_color: Colors,
}

/// `List` is a set of poor orphan children, they don't have any style, just blowing in the wind.
pub struct List {
    pub children: Vec<Tree>,
}

/// `SizedBox` just has width and height two arguments, we use this component to take some white space usually.
pub struct SizedBox {
    pub child: Tree,
    pub style: SizedBoxStyle,
}

/// `SizedBox` style
pub struct SizedBoxStyle {
    pub height: Unit,
    pub width: Unit,
}

// Flex
/// `Align` inherits the core usage of Alignments, it's quite simple, just one property.
pub struct Align {
    pub child: Tree,
    pub style: AlignStyle,
}

/// `Align` style
pub struct AlignStyle {
    pub align: Alignments,
}

/// `Center` is a very nice widget, if your website only have a line of chars, use it!
pub struct Center {
    pub child: Tree,
}

/// `Col` is the typical flow in html, with flexible enhance.
pub struct Col {
    pub children: Vec<Tree>,
}

/// This is the Lunatic Widget to Ground Control, 'I`m stepping throw the Window.'
pub struct Flex {
    pub child: Tree,
    pub style: FlexStyle,
}

/// `Flex` Style
pub struct FlexStyle {
    pub align: Alignments,
    pub basis: FlexBasis,
    pub direction: FlexDirection,
    pub grow: Unit,
    pub order: Unit,
    pub wrap: bool,
}

/// Both `Col` and `Row` are using flex-start, if you want to reverse the children of them, better to work on the list order.
pub struct Row {
    pub children: Vec<Tree>,
}

// Grid
/// `Grid` is quite complex in some way, usually, we just `Grid` our contains.
pub struct Grid {
    pub children: Vec<Tree>,
    pub style: GridStyle,
}

/// `Grid` Style
pub struct GridStyle {
    pub col: GridAuto,
    pub col_gap: Unit,
    pub flow: GridFlow,
    pub row: GridAuto,
    pub row_gap: Unit,
    pub template_col: GridTemplate,
    pub template_row: GridTemplate,
}

// MultiColumn
/// **Homework**: code a New York Times.
pub struct MultiColumn {
    pub children: Vec<Tree>,
    pub style: MultiColumnStyle,
}

/// `Multicolumn` Style
pub struct MultiColumnStyle {
    pub color: Colors,
    pub count: Unit,
    pub gap: Unit,
    pub style: MultiColumnLineStyle,
}
