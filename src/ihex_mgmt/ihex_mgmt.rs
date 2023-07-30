//mod file_mgmt;

//pub use crate::file_mgmt::read_file_content_as_string;
//pub mod file_mgmt;

use crate::file_mgmt::file_mgmt::read_file_content_as_string;

use ihex::Record;
use ihex::types;

pub struct Segment {
    pub address: u16,
    pub size: u32,
    pub data: Vec<u8>,
}

impl Segment {
    pub fn new() -> Segment {
        Segment {
            address: 0x00,
            size: 0u32,
            data: Vec::new(),
        }
    }

    // pub fn new(address: u16) -> Segment {
    //     Segment {
    //         address: address,
    //         size: 0u32,
    //         data: Vec::new(),
    //     }
    // }
}

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\nAddress: {:02X?}", self.address)?;
        write!(f, "\nSize: {:02X?}", self.size)?;
        write!(f, "\nData: \n{:02X?}", self.data)
    }
}

#[allow(dead_code, unused)]
pub fn parse_hex_file(segments: &mut Vec<Segment>, path: &String) -> Result<String, Box<dyn std::error::Error>>
{
    log::info!("parse_hex_file() path: {:?} ...", path);

    let mut idx: usize = 0;

    match read_file_content_as_string(path) {

        Ok(data) => {

            let mut reader = ihex::Reader::new(&data);

            while let Some(record_result) = reader.next() {

                // https://stackoverflow.com/questions/63859927/how-to-get-a-value-from-a-result
                let record: Record = record_result.unwrap();

                match record.record_type() {

                    types::DATA => {
                        log::trace!("types::DATA");

                        // some hex files contain no segments. Data is read before the first
                        // segment is created. Handle this case here.
                        if idx == 0 {
                            // first add a dummy segment if this ihex file contains no segments!
                            let segment: Segment = Segment::new();
                            segments.push(segment);

                            idx += 1;
                        }

                        if let Record::Data{ offset: _, value } = record {
                            for temp_data in &value {
                                segments[idx-1].data.push(temp_data.clone());
                            }
                        }
                    },

                    types::END_OF_FILE => log::trace!("types::END_OF_FILE"),

                    types::EXTENDED_SEGMENT_ADDRESS => {
                        if let Record::ExtendedSegmentAddress(address) = record {
                            log::trace!("ExtendedSegmentAddress: {:02X?}", address);
                        }
                    },

                    types::START_SEGMENT_ADDRESS => {
                        log::trace!("types::START_SEGMENT_ADDRESS");
                        if let Record::StartSegmentAddress{ cs, ip } = record {
                            log::trace!("StartSegmentAddress: cs: {:02X?}, ip: {:02X?}", cs, ip);
                        }
                    } 

                    types::EXTENDED_LINEAR_ADDRESS => {
                        if let Record::ExtendedLinearAddress(address) = record {
                            log::trace!("ExtendedLinearAddress: {:02X?}", address);

                            idx += 1;

                            let mut segment: Segment = Segment::new();
                            segment.address = address;
                            segments.push(segment);

                            log::trace!("Segment.Len: {}", segments.len());
                        }
                    },

                    types::START_LINEAR_ADDRESS => {
                        if let Record::StartLinearAddress(address) = record {
                            log::trace!("StartLinearAddress: {:02X?}", address);
                        }
                    }

                    _ => log::error!("Unknown"),

                }
            }
        }
        Err(e) => Err(e)?
    }

    // update each segments size
    for seg in segments.iter_mut() {
        seg.size = seg.data.len() as u32;
        log::trace!("Segment: {}", seg);
    }

    log::trace!("parse_hex_file() segments: {:?} ...", segments.len());

    Ok("success".to_string())

}