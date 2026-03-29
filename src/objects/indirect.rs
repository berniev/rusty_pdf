/// PDF Spec:
///
/// Indirect is a wrapper, not a peer variant
///
/// Any object in a PDF file may be labelled as an indirect object. This gives the object a
/// unique object identifier by which other objects can refer to it (for example, as an
/// element of an array or as the value of a dictionary entry).
///
/// An object identifier shall consist of two parts:
/// - A positive integer object number. Indirect objects may be numbered sequentially
///   within a PDF file, but this is not required; object numbers may be assigned in any
///   arbitrary order.
/// - A non-negative integer generation number. In a newly created file, all indirect
///   objects shall have generation numbers of 0. Nonzero generation numbers will be
///   introduced when the file is later updated.
///
///       Example: {obj_num} {gen_num} obj {object} endobj
///
/// Together, the combination of an object number and a generation number shall uniquely
/// identify an indirect object.
///
use crate::PdfObject;

//-------------------------- PdfIndirectObject ----------------------//

pub enum HostType {
    Standard { generation_number: u16 },
    ObjectStream { stream_obj_num: usize }, // v1.5+, stream_obj_num is obj num of containing ObjStm
}

pub struct PdfIndirectObject {
    pub obj_num: usize,
    pub storage: HostType,
    
    /// HostType       byte_offset from
    /// =============  ================
    /// Standard       start of file
    /// ObjectStream  `First` in the object stream
    pub byte_offset: usize,
}

impl PdfIndirectObject {
    pub fn new_standard(obj_num: usize) -> Self {
        Self {
            obj_num,
            storage: HostType::Standard {
                generation_number: 0,
            },
            byte_offset: 0,
        }
    }

    pub fn new_in_obj_stream(
        obj_num: usize,
        stream_obj_num: usize,
    ) -> Self {
        Self {
            obj_num,
            storage: HostType::ObjectStream { stream_obj_num },
            byte_offset: 0,
        }
    }

    pub fn reference(&self) -> Vec<u8> {
        let gen_num = match &self.storage {
            HostType::Standard { generation_number } => *generation_number,
            HostType::ObjectStream { .. } => 0,
        };
        format!("{} {} R", self.obj_num, gen_num).into_bytes()
    }
}

impl PdfObject for PdfIndirectObject {
    fn serialise(&mut self) -> Vec<u8> {
        match &self.storage {
            HostType::Standard { generation_number } => {
                let gen_num = *generation_number;
                let mut out = format!("{} {} obj\n", self.obj_num, gen_num).into_bytes();
                //out.extend(self.object_being_wrapped.serialise());
                out.extend(b"\nendobj\n");
                out
            }
            HostType::ObjectStream { .. } => self.object_being_wrapped.serialise(),
        }
    }
}

struct PdfIndirectRef{
    
}