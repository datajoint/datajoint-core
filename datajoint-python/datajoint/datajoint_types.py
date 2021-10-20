from .datajoint_core_lib import dj_core
from enum import Enum


class DataJointType():
    Unknown = dj_core.DataJointType_Unknown
    TinyInt = dj_core.DataJointType_TinyInt
    TinyIntUnsigned = dj_core.DataJointType_TinyIntUnsigned
    SmallInt = dj_core.DataJointType_SmallInt
    SmallIntUnsigned = dj_core.DataJointType_SmallIntUnsigned
    MediumInt = dj_core.DataJointType_MediumInt
    MediumIntUnsigned = dj_core.DataJointType_MediumIntUnsigned
    Int = dj_core.DataJointType_Int
    IntUnsigned = dj_core.DataJointType_IntUnsigned
    Enum = dj_core.DataJointType_Enum
    Date = dj_core.DataJointType_Date
    Time = dj_core.DataJointType_Time
    DateTime = dj_core.DataJointType_DateTime
    Timestamp = dj_core.DataJointType_Timestamp
    CharN = dj_core.DataJointType_CharN
    VarCharN = dj_core.DataJointType_VarCharN
    Float = dj_core.DataJointType_Float
    Double = dj_core.DataJointType_Double
    Decimal = dj_core.DataJointType_Decimal
    TinyBlob = dj_core.DataJointType_TinyBlob
    MediumBlob = dj_core.DataJointType_MediumBlob
    Blob = dj_core.DataJointType_Blob
    LongBlob = dj_core.DataJointType_LongBlob
    Attach = dj_core.DataJointType_Attach
    FilepathStore = dj_core.DataJointType_FilepathStore
