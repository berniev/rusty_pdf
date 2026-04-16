use crate::{PdfArrayObject, PdfDictionaryObject, PdfError, PdfObject};
//--------------------------- ShadingType ----------------------//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadingType {
    Function = 1,
    Axial = 2,
    Radial = 3,
    FreeFormGouraud = 4,
    LatticeGouraud = 5,
    CoonsPatch = 6,
    TensorPatch = 7,
}

//--------------------------- ShadingBase ----------------------//

pub trait ShadingBase {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject;

    fn with_background(mut self, background: PdfObject) -> Result<Self, PdfError>
    where
        Self: Sized,
    {
        self.dict_mut().add("Background", background)?;

        Ok(self)
    }

    fn with_bbox(mut self, bbox: PdfObject) -> Result<Self, PdfError>
    where
        Self: Sized,
    {
        self.dict_mut().add("BBox", bbox)?;

        Ok(self)
    }

    fn with_anti_alias(mut self, value: bool) -> Result<Self, PdfError>
    where
        Self: Sized,
    {
        self.dict_mut().add("AntiAlias", value)?;

        Ok(self)
    }
}

//--------------------------- builder ----------------------//

fn make_shading(color_space: PdfObject, shading_type: ShadingType) -> Result<PdfDictionaryObject, PdfError> {
    let mut dict = PdfDictionaryObject::new();
    dict.add("ShadingType", shading_type as i64)?;
    dict.add("ColorSpace", color_space)?;

    Ok(dict)
}

//--------------------------- FunctionShading (1) ----------------------//

pub struct Shading1Function {
    dictionary: PdfDictionaryObject,
}

impl ShadingBase for Shading1Function {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject {
        &mut self.dictionary
    }
}

impl Shading1Function {
    pub fn new(color_space: PdfObject, function: PdfDictionaryObject) -> Result<Self, PdfError> {
        let mut dictionary = make_shading(color_space, ShadingType::Function)?;
        dictionary.add("Function", function)?;

        Ok(Self { dictionary })
    }

    pub fn with_domain(mut self, domain: PdfArrayObject) -> Result<Self, PdfError> {
        self.dictionary.add("Domain", domain)?;

        Ok(self)
    }

    pub fn with_matrix(mut self, matrix: PdfArrayObject) -> Result<Self, PdfError> {
        self.dictionary.add("Matrix", matrix)?;

        Ok(self)
    }
}

//--------------------------- AxialShading (2) ----------------------//

pub struct Shading2Axial {
    dictionary: PdfDictionaryObject,
}

impl ShadingBase for Shading2Axial {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject {
        &mut self.dictionary
    }
}

impl Shading2Axial {
    pub fn new(
        color_space: PdfObject,
        coords: PdfArrayObject,
        function: PdfDictionaryObject,
    ) -> Result<Self, PdfError> {
        let mut dictionary = make_shading(color_space, ShadingType::Axial)?;
        dictionary.add("Coords", coords)?;
        dictionary.add("Function", function)?;

        Ok(Self { dictionary })
    }

    pub fn with_domain(mut self, domain: PdfArrayObject) -> Result<Self, PdfError> {
        self.dictionary.add("Domain", domain)?;

        Ok(self)
    }

    pub fn with_extend(mut self, extend: PdfArrayObject) -> Result<Self, PdfError> {
        self.dictionary.add("Extend", extend)?;

        Ok(self)
    }
}

//--------------------------- RadialShading (3) ----------------------//

pub struct Shading3Radial {
    dictionary: PdfDictionaryObject,
}

impl ShadingBase for Shading3Radial {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject {
        &mut self.dictionary
    }
}

impl Shading3Radial {
    pub fn new(color_space: PdfObject, function: PdfDictionaryObject) -> Result<Self, PdfError> {
        let mut dictionary = make_shading(color_space, ShadingType::Radial)?;
        dictionary.add("Function", function)?;

        Ok(Self { dictionary })
    }

    pub fn with_domain(mut self, domain: PdfArrayObject) -> Result<Self,PdfError> {
        self.dictionary.add("Domain", domain)?;

        Ok(self)
    }

    pub fn with_extend(mut self, extend: PdfArrayObject) -> Result<Self, PdfError> {
        self.dictionary.add("Extend", extend)?;

        Ok(self)
    }
}

//--------------------------- FreeFormGouraudShading (4) ----------------------//

pub struct Shading4FreeFormGouraud {
    dictionary: PdfDictionaryObject,
}

impl ShadingBase for Shading4FreeFormGouraud {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject {
        &mut self.dictionary
    }
}

impl Shading4FreeFormGouraud {
    pub fn new(
        color_space: PdfObject,
        bits_per_coordinate: u64,
        bits_per_component: u64,
        bits_per_flag: u64,
        decode: PdfArrayObject,
    ) -> Result<Self, PdfError> {
        let mut dictionary = make_shading(color_space, ShadingType::FreeFormGouraud)?;
        dictionary.add("BitsPerCordinate", bits_per_component)?;
        dictionary.add("BitsPerComponent", bits_per_coordinate)?;
        dictionary.add("BitsPerFlag", bits_per_flag)?;
        dictionary.add("Decode", decode)?;

        Ok(Self { dictionary })
    }

    pub fn with(mut self, function: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dictionary.add("Function", function)?;

        Ok(self)
    }
}

//--------------------------- LatticeGouraudShading (5) ----------------------//

pub struct Shading5LatticeGouraud {
    dictionary: PdfDictionaryObject,
}

impl ShadingBase for Shading5LatticeGouraud {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject {
        &mut self.dictionary
    }
}

impl Shading5LatticeGouraud {
    pub fn new(
        color_space: PdfObject,
        bits_per_coordinate: u64,
        bits_per_component: u64,
        vertices_per_row: u64,
        decode: PdfArrayObject,
    ) -> Result<Self, PdfError> {
        let mut dictionary = make_shading(color_space, ShadingType::LatticeGouraud)?;
        dictionary.add("BitsPerCordinate", bits_per_component)?;
        dictionary.add("BitsPerComponent", bits_per_coordinate)?;
        dictionary.add("VerticesPerRow", vertices_per_row)?;
        dictionary.add("Decode", decode)?;

        Ok(Self { dictionary })
    }

    pub fn with(mut self, function: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dictionary.add("Function", function)?;

        Ok(self)
    }
}

//--------------------------- patch shading ----------------------//

fn make_patch_shading(
    color_space: PdfObject,
    shading_type: ShadingType,
    bits_per_coordinate: i64,
    bits_per_component: i64,
    bits_per_flag: i64,
) -> Result<PdfDictionaryObject, PdfError> {
    let mut dictionary = make_shading(color_space, shading_type)?;
    dictionary.add("BitsPerCoordinate", bits_per_coordinate)?;
    dictionary.add("BitsPerComponent", bits_per_component)?;
    dictionary.add("BitsPerFlag", bits_per_flag)?;

    Ok(dictionary)
}

//-------------------- CoonsPatchShading (6) -----------------------------------//

pub struct Shading6CoonsPatch {
    dictionary: PdfDictionaryObject,
}

impl ShadingBase for Shading6CoonsPatch {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject {
        &mut self.dictionary
    }
}

impl Shading6CoonsPatch {
    pub fn new(
        color_space: PdfObject,
        bits_per_coordinate: i64,
        bits_per_component: i64,
        bits_per_flag: i64,
    ) -> Result<Self, PdfError> {
        Ok(Self {
            dictionary: make_patch_shading(
                color_space,
                ShadingType::CoonsPatch,
                bits_per_coordinate,
                bits_per_component,
                bits_per_flag,
            )?,
        })
    }

    pub fn with_decode(mut self, decode: PdfArrayObject) -> Result<Self, PdfError> {
        self.dictionary.add("Decode", decode)?;

        Ok(self)
    }

    pub fn with_function(mut self, function: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dictionary.add("Function", function)?;

        Ok(self)
    }
}

//-------------------- TensorPatchShading (7) -----------------------------------//

pub struct Shading7TensorPatch {
    dictionary: PdfDictionaryObject,
}

impl ShadingBase for Shading7TensorPatch {
    fn dict_mut(&mut self) -> &mut PdfDictionaryObject {
        &mut self.dictionary
    }
}

impl Shading7TensorPatch {
    pub fn new(
        color_space: PdfObject,
        bits_per_coordinate: i64,
        bits_per_component: i64,
        bits_per_flag: i64,
    ) -> Result<Self, PdfError> {
        Ok(Self {
            dictionary: make_patch_shading(
                color_space,
                ShadingType::TensorPatch,
                bits_per_coordinate,
                bits_per_component,
                bits_per_flag,
            )?,
        })
    }

    pub fn with_decode(mut self, decode: PdfArrayObject) -> Result<Self, PdfError> {
        self.dictionary.add("Decode", decode)?;

        Ok(self)
    }

    pub fn with_function(mut self, function: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dictionary.add("Function", function)?;

        Ok(self)
    }
}
