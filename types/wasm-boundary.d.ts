export type ActivityCard = 'Card' | 'Vu';

export type ActivitySource = 'Automatic' | 'Manual' | 'Unknown';

export type ActivityType =
    | 'Availability'
    | 'Driving'
    | 'Rest'
    | 'Unknown'
    | 'Work';

export type CalibrationPurpose =
    | 'Activation'
    | 'FirstInstallation'
    | 'Installation'
    | 'PeriodicInspection'
    | 'Reserved'
    | 'TimeAdjustmentWithoutCalibration'
    | 'Unknown'
    | 'VRNEntryByCompany';

export type CardSlotNumber = 'CoDriver' | 'Driver' | 'Unknown';

export type CardStatus = 'Inserted' | 'Removed' | 'Unknown';

export type ControlType =
    | 'CalibrationParameters'
    | 'CardDownloaded'
    | 'DisplayUsed'
    | 'PrintingDone'
    | 'Unknown'
    | 'VUDownloaded';

export type DrivingStatus = 'CrowOrKnown' | 'SingleOrUnknown' | 'Unknown';

export type EntryTypeDailyWorkPeriod =
    | 'BeginAssumedByVU'
    | 'BeginCardInsertion'
    | 'BeginGnssData'
    | 'BeginManuallyEntered'
    | 'EndAssumedByVU'
    | 'EndCardWithdrawal'
    | 'EndGnssData'
    | 'EndManuallyEntered'
    | 'Unknown';

export type EquipmentType =
    | 'CompanyCard'
    | 'ControlCard'
    | 'DriverCard'
    | 'NullCard'
    | 'Unknown'
    | 'WorkshopCard';

export type EventFaultRecordPurpose = 'ActiveEventOrFault' | 'Unknown';

export type EventFaultType = 'PowerSupplyInterruption' | 'Unknown' | string;

export type RegionNumeric = string;

export type SpecificConditionType =
    | 'FerryTrainCrossing'
    | 'FerryTrainCrossingEnd'
    | 'OutOfScopeBegin'
    | 'OutOfScopeEnd'
    | 'Unknown';

export function getSupportedEnumValues(): unknown;
export function getSupportedNationAlphaCodes(): unknown;
export function parse_from_memory(esmData: Uint8Array): unknown;
export function verify_card(
    generation: CardGeneration,
    dataFilesMap: unknown,
    ercaPublicKey: Uint8Array,
): unknown;
