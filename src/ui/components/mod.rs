pub mod editor;
pub mod filename_prompt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FocusableComponent {
    Editor,
    FilenamePrompt,
}
