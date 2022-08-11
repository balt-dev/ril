window.SIDEBAR_ITEMS = {"enum":[["BorderPosition","Represents whether a border is inset, outset, or if it lays in the center."],["DisposalMethod","The method used to dispose a frame before transitioning to the next frame in an image sequence."],["Dynamic","Represents a pixel type that is dynamically resolved."],["DynamicFrameIterator","Represents any one of the different types of frame iterators, compacted into one common enum with common methods."],["Error","Represents an error that occurs within the crate."],["HorizontalAnchor","Represents where text is anchored horizontally."],["ImageFormat","Represents the underlying encoding format of an image."],["LoopCount","Determines how many times an image sequence should repeat itself, or if it should repeat infinitely."],["OverlayMode","The behavior to use when overlaying images on top of each other."],["ResizeAlgorithm","A filtering algorithm that is used to resize an image."],["VerticalAnchor","Represents where text is anchored vertically."],["WrapStyle","Determines how text should be wrapped."]],"mod":[["draw","Encloses most drawing implementations and drawable objects."],["encode","Houses Encoder, Decoder, and frame iterator traits."],["encodings","Contains encoder and decoder implementations for various image formats."],["error","Common error types."],["pixel","Encloses pixel-related traits and pixel type implementations."],["prelude","The crate prelude exports. Importing this with a wildcard will import most items from RIL that can be useful for image processing, along with bringing crucial traits into scope."],["sequence","Implements the animated image and image sequence interface."],["text","Implements the font/text rasterizing and layout interface."]],"struct":[["BitPixel","Represents a single-bit pixel that represents either a pixel that is on or off."],["Border","Represents a shape border."],["Ellipse","An ellipse, which could be a circle."],["Font","Represents a single font along with its alternatives used to render text. Currently, this supports TrueType and OpenType fonts."],["Frame","Represents a frame in an image sequence. It encloses an [`Image`] and extra metadata about the frame."],["Image","A high-level image representation."],["ImageSequence","Represents a sequence of image frames such as an animated image."],["L","Represents an L, or luminance pixel that is stored as only one single number representing how bright, or intense, the pixel is."],["Paste","Pastes or overlays an image on top of another image."],["Rectangle","A rectangle."],["Rgb","Represents an RGB pixel."],["Rgba","Represents an RGBA pixel."],["TextLayout","Represents a high-level text layout that can layout text segments, maybe with different fonts."],["TextSegment","Represents a text segment that can be drawn."]],"trait":[["Alpha","Represents a pixel that supports alpha, or transparency values."],["Banded","Represents an image with multiple channels, called bands."],["Decoder","Low-level decoder interface around an image format."],["Draw","A common trait for all objects able to be drawn on an image."],["Encoder","Low-level encoder interface around an image format."],["FrameIterator","Represents the lazy decoding of frames from an encoded image sequence, such as an animated image."],["Pixel","Represents any type of pixel in an image."]],"type":[["Result","A shortcut type equivalent to `Result<T, ril::Error>`."]]};