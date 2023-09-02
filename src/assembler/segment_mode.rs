#[derive(PartialEq)]
#[derive(Debug, Copy, Clone)]
#[derive(Default)]
pub enum SegmentMode {
    
    CodeSegment,
    DataSegment,

    #[default]
    UNKNOWN,
}