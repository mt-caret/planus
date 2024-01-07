(** The root namespace

    Generated from these locations:
    * File `/Users/delta/dev/misc/arrow/format/File.fbs` *)

module Root = struct
  (** The namespace `org`

      Generated from these locations:
      * File `/Users/delta/dev/misc/arrow/format/File.fbs` *)

  module Org = struct
    (** The namespace `org.apache`

        Generated from these locations:
        * File `/Users/delta/dev/misc/arrow/format/File.fbs` *)

    module Apache = struct
      (** The namespace `org.apache.arrow`

          Generated from these locations:
          * File `/Users/delta/dev/misc/arrow/format/File.fbs` *)

      module Arrow = struct
        (** The namespace `org.apache.arrow.flatbuf`

            Generated from these locations:
            * File `/Users/delta/dev/misc/arrow/format/File.fbs`
            * File `/Users/delta/dev/misc/arrow/format/Schema.fbs`
            * File `/Users/delta/dev/misc/arrow/format/Message.fbs`
            * File `/Users/delta/dev/misc/arrow/format/SparseTensor.fbs`
            * File `/Users/delta/dev/misc/arrow/format/Tensor.fbs` *)

        module Flatbuf = struct
          (** The enum `MetadataVersion` in the namespace `org.apache.arrow.flatbuf`

              Generated from these locations:
              * Enum `MetadataVersion` in the file `/Users/delta/dev/misc/arrow/format/Schema.fbs:30` *)
          type metadata_version =
            | V1 (** The variant `V1` in the enum `MetadataVersion` *)
            | V2 (** The variant `V2` in the enum `MetadataVersion` *)
            | V3 (** The variant `V3` in the enum `MetadataVersion` *)
            | V4 (** The variant `V4` in the enum `MetadataVersion` *)
            | V5 (** The variant `V5` in the enum `MetadataVersion` *)

          (** The enum `Feature` in the namespace `org.apache.arrow.flatbuf`

              Generated from these locations:
              * Enum `Feature` in the file `/Users/delta/dev/misc/arrow/format/Schema.fbs:70` *)
          type feature =
            | Unused (** The variant `UNUSED` in the enum `Feature` *)
            | Dictionary_replacement
            (** The variant `DICTIONARY_REPLACEMENT` in the enum `Feature` *)
            | Compressed_body (** The variant `COMPRESSED_BODY` in the enum `Feature` *)

          (** The enum `UnionMode` in the namespace `org.apache.arrow.flatbuf`

              Generated from these locations:
              * Enum `UnionMode` in the file `/Users/delta/dev/misc/arrow/format/Schema.fbs:147` *)
          type union_mode =
            | Sparse (** The variant `Sparse` in the enum `UnionMode` *)
            | Dense (** The variant `Dense` in the enum `UnionMode` *)

          (** The enum `Precision` in the namespace `org.apache.arrow.flatbuf`

              Generated from these locations:
              * Enum `Precision` in the file `/Users/delta/dev/misc/arrow/format/Schema.fbs:163` *)
          type precision =
            | Half (** The variant `HALF` in the enum `Precision` *)
            | Single (** The variant `SINGLE` in the enum `Precision` *)
            | Double (** The variant `DOUBLE` in the enum `Precision` *)

          (** The enum `DateUnit` in the namespace `org.apache.arrow.flatbuf`

              Generated from these locations:
              * Enum `DateUnit` in the file `/Users/delta/dev/misc/arrow/format/Schema.fbs:240` *)
          type date_unit =
            | Day (** The variant `DAY` in the enum `DateUnit` *)
            | Millisecond (** The variant `MILLISECOND` in the enum `DateUnit` *)

          (** The enum `TimeUnit` in the namespace `org.apache.arrow.flatbuf`

              Generated from these locations:
              * Enum `TimeUnit` in the file `/Users/delta/dev/misc/arrow/format/Schema.fbs:255` *)
          type time_unit =
            | Second (** The variant `SECOND` in the enum `TimeUnit` *)
            | Millisecond (** The variant `MILLISECOND` in the enum `TimeUnit` *)
            | Microsecond (** The variant `MICROSECOND` in the enum `TimeUnit` *)
            | Nanosecond (** The variant `NANOSECOND` in the enum `TimeUnit` *)

          (** The enum `IntervalUnit` in the namespace `org.apache.arrow.flatbuf`

              Generated from these locations:
              * Enum `IntervalUnit` in the file `/Users/delta/dev/misc/arrow/format/Schema.fbs:397` *)
          type interval_unit =
            | Year_month (** The variant `YEAR_MONTH` in the enum `IntervalUnit` *)
            | Day_time (** The variant `DAY_TIME` in the enum `IntervalUnit` *)
            | Month_day_nano
            (** The variant `MONTH_DAY_NANO` in the enum `IntervalUnit` *)

          (** The union `Type` in the namespace `org.apache.arrow.flatbuf`

              Generated from these locations:
              * Union `Type` in the file `/Users/delta/dev/misc/arrow/format/Schema.fbs:441` *)
          type type_ =
            | Null (** The variant of type `Null` in the union `Type` *)
            | Int (** The variant of type `Int` in the union `Type` *)
            | Floating_point
            (** The variant of type `FloatingPoint` in the union `Type` *)
            | Binary (** The variant of type `Binary` in the union `Type` *)
            | Utf8 (** The variant of type `Utf8` in the union `Type` *)
            | Bool (** The variant of type `Bool` in the union `Type` *)
            | Decimal (** The variant of type `Decimal` in the union `Type` *)
            | Date (** The variant of type `Date` in the union `Type` *)
            | Time (** The variant of type `Time` in the union `Type` *)
            | Timestamp (** The variant of type `Timestamp` in the union `Type` *)
            | Interval (** The variant of type `Interval` in the union `Type` *)
            | List (** The variant of type `List` in the union `Type` *)
            | Struct (** The variant of type `Struct_` in the union `Type` *)
            | Union (** The variant of type `Union` in the union `Type` *)
            | Fixed_size_binary
            (** The variant of type `FixedSizeBinary` in the union `Type` *)
            | Fixed_size_list
            (** The variant of type `FixedSizeList` in the union `Type` *)
            | Map (** The variant of type `Map` in the union `Type` *)
            | Duration (** The variant of type `Duration` in the union `Type` *)
            | Large_binary (** The variant of type `LargeBinary` in the union `Type` *)
            | Large_utf8 (** The variant of type `LargeUtf8` in the union `Type` *)
            | Large_list (** The variant of type `LargeList` in the union `Type` *)
            | Run_end_encoded
            (** The variant of type `RunEndEncoded` in the union `Type` *)
            | Binary_view (** The variant of type `BinaryView` in the union `Type` *)
            | Utf8_view (** The variant of type `Utf8View` in the union `Type` *)
            | List_view (** The variant of type `ListView` in the union `Type` *)
            | Large_list_view
            (** The variant of type `LargeListView` in the union `Type` *)

          (** The enum `DictionaryKind` in the namespace `org.apache.arrow.flatbuf`

              Generated from these locations:
              * Enum `DictionaryKind` in the file `/Users/delta/dev/misc/arrow/format/Schema.fbs:484` *)
          type dictionary_kind =
            | Dense_array (** The variant `DenseArray` in the enum `DictionaryKind` *)

          (** The enum `Endianness` in the namespace `org.apache.arrow.flatbuf`

              Generated from these locations:
              * Enum `Endianness` in the file `/Users/delta/dev/misc/arrow/format/Schema.fbs:535` *)
          type endianness =
            | Little (** The variant `Little` in the enum `Endianness` *)
            | Big (** The variant `Big` in the enum `Endianness` *)

          (** The enum `CompressionType` in the namespace `org.apache.arrow.flatbuf`

              Generated from these locations:
              * Enum `CompressionType` in the file `/Users/delta/dev/misc/arrow/format/Message.fbs:45` *)
          type compression_type =
            | Lz4_frame (** The variant `LZ4_FRAME` in the enum `CompressionType` *)
            | Zstd (** The variant `ZSTD` in the enum `CompressionType` *)

          (** The enum `BodyCompressionMethod` in the namespace `org.apache.arrow.flatbuf`

              Generated from these locations:
              * Enum `BodyCompressionMethod` in the file `/Users/delta/dev/misc/arrow/format/Message.fbs:58` *)
          type body_compression_method =
            | Buffer (** The variant `BUFFER` in the enum `BodyCompressionMethod` *)

          (** The union `MessageHeader` in the namespace `org.apache.arrow.flatbuf`

              Generated from these locations:
              * Union `MessageHeader` in the file `/Users/delta/dev/misc/arrow/format/Message.fbs:146` *)
          type message_header =
            | Schema (** The variant of type `Schema` in the union `MessageHeader` *)
            | Dictionary_batch
            (** The variant of type `DictionaryBatch` in the union `MessageHeader` *)
            | Record_batch
            (** The variant of type `RecordBatch` in the union `MessageHeader` *)
            | Tensor (** The variant of type `Tensor` in the union `MessageHeader` *)
            | Sparse_tensor
            (** The variant of type `SparseTensor` in the union `MessageHeader` *)

          (** The enum `SparseMatrixCompressedAxis` in the namespace `org.apache.arrow.flatbuf`

              Generated from these locations:
              * Enum `SparseMatrixCompressedAxis` in the file `/Users/delta/dev/misc/arrow/format/SparseTensor.fbs:77` *)
          type sparse_matrix_compressed_axis =
            | Row (** The variant `Row` in the enum `SparseMatrixCompressedAxis` *)
            | Column (** The variant `Column` in the enum `SparseMatrixCompressedAxis` *)

          (** The union `SparseTensorIndex` in the namespace `org.apache.arrow.flatbuf`

              Generated from these locations:
              * Union `SparseTensorIndex` in the file `/Users/delta/dev/misc/arrow/format/SparseTensor.fbs:203` *)
          type sparse_tensor_index =
            | Sparse_tensor_index_coo
            (** The variant of type `SparseTensorIndexCOO` in the union `SparseTensorIndex` *)
            | Sparse_matrix_index_csx
            (** The variant of type `SparseMatrixIndexCSX` in the union `SparseTensorIndex` *)
            | Sparse_tensor_index_csf
            (** The variant of type `SparseTensorIndexCSF` in the union `SparseTensorIndex` *)
        end
      end
    end
  end
end
