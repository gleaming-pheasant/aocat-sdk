use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};

use crate::{Error, types::Uri};

/// Serves as a builder, but can be built from new. Each image is built into a private controlled
/// type when added to the `Images` list (which is not mandatory, and has no minimum length).
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Image {
    image_url: Option<String>,
    iiif_manifest_url: Option<String>,
    primary: bool, // default is `false`
}

impl Image {
    pub fn new(
        image_url: Option<String>,
        iiif_manifest_url: Option<String>,
        primary: bool,
    ) -> Self {
        Image {
            image_url,
            iiif_manifest_url,
            primary,
        }
    }

    pub fn set_image_url(&mut self, image_url: Option<String>) {
        self.image_url = image_url;
    }

    pub fn set_iiif_manifest_url(&mut self, iiif_manifest_url: Option<String>) {
        self.iiif_manifest_url = iiif_manifest_url;
    }

    pub fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    pub fn image_url(&self) -> Option<&str> {
        self.image_url.as_deref()
    }

    pub fn iiif_manifest_url(&self) -> Option<&str> {
        self.iiif_manifest_url.as_deref()
    }

    pub fn primary(&self) -> bool {
        self.primary
    }
}

#[derive(Debug)]
enum InnerImage<'a> {
    Image(Uri<'a>),
    IiifManifest(Uri<'a>),
    Both {
        image_url: Uri<'a>,
        iiif_manifest_url: Uri<'a>,
    },
}

/// with a lifetime to prevent cloning of the contained values, but otherwise the exact same
/// structure as `Image`. Acts as a DTO for serializing.
#[derive(Serialize)]
struct ImageOut<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<&'a Uri<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iiif_manifest_url: Option<&'a Uri<'a>>,
    primary: bool,
}

impl<'a> InnerImage<'a> {
    fn as_out(&'a self, primary: bool) -> ImageOut<'a> {
        match self {
            InnerImage::Image(i) => ImageOut {
                image_url: Some(i),
                iiif_manifest_url: None,
                primary,
            },
            InnerImage::IiifManifest(m) => ImageOut {
                image_url: None,
                iiif_manifest_url: Some(m),
                primary,
            },
            InnerImage::Both {
                image_url,
                iiif_manifest_url,
            } => ImageOut {
                image_url: Some(image_url),
                iiif_manifest_url: Some(iiif_manifest_url),
                primary,
            },
        }
    }
}

impl<'a> TryFrom<Image> for InnerImage<'a> {
    type Error = Error;

    fn try_from(value: Image) -> Result<Self, Self::Error> {
        match (value.image_url(), value.iiif_manifest_url()) {
            (Some(image_url), Some(iiif_manifest_url)) => Ok(InnerImage::Both {
                image_url: Uri::try_from(image_url.to_string())?,
                iiif_manifest_url: Uri::try_from(iiif_manifest_url.to_string())?,
            }),
            (Some(image_url), None) => Ok(InnerImage::Image(Uri::try_from(image_url.to_string())?)),
            (None, Some(iiif_manifest_url)) => Ok(InnerImage::IiifManifest(Uri::try_from(
                iiif_manifest_url.to_string(),
            )?)),
            (None, None) => Err(Error::MissingField("image_url or iiif_manifest_url")),
        }
    }
}

#[derive(Debug, Default)]
pub struct Images<'a> {
    images: Vec<InnerImage<'a>>,
    primary_index: usize,
}

impl<'a> Serialize for Images<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.images.len()))?;
        for (ix, image) in self.images.iter().enumerate() {
            seq.serialize_element(&image.as_out(ix == self.primary_index))?;
        }
        seq.end()
    }
}

impl<'a> TryFrom<Vec<Image>> for Images<'a> {
    type Error = crate::Error;

    fn try_from(v: Vec<Image>) -> Result<Self, Self::Error> {
        let mut images = Vec::with_capacity(v.len());
        let mut primary_index: Option<usize> = None;

        for (ix, image) in v.into_iter().enumerate() {
            if image.primary() == true {
                if primary_index.is_some() {
                    return Err(Error::MultiplePrimaryImages);
                }
                primary_index = Some(ix);
            }

            images.push(InnerImage::try_from(image)?);
        }

        if primary_index.is_none() && !images.is_empty() {
            return Err(Error::NoPrimaryImages);
        }

        Ok(Images {
            images,
            primary_index: primary_index.unwrap_or(0),
        })
    }
}
