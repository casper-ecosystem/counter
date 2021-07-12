//@ts-nocheck
import {Error, ErrorCode} from "casper-contract/error";
import {getKey,
        putKey,
        ret,
        EntryPoint, 
        EntryPoints,
        EntryPointAccess,
        PublicAccess,
        EntryPointType,
        newLockedContract,
        AddContractVersionResult} from "casper-contract/index";
import {Key} from "casper-contract/key";
import {Pair} from "casper-contract/pair";
import {fromBytesLoad} from "casper-contract/bytesrepr";
import {CLValue,CLType,CLTypeTag}  from "casper-contract/clvalue";

const COUNT_KEY: string = "count";
const COUNTER_INC: string= "counter_inc";
const COUNTER_GET: string= "counter_get";
const COUNTER_KEY: string = "counter";

export function counter_inc():void {
    let uref= getKey(COUNT_KEY);
    if(uref === null){
        Error.fromErrorCode(ErrorCode.GetKey).revert();
    } else {
        let uref_key  = <Key>uref;
        uref_key.add(CLValue.fromI32(<i32>1));
    }
}

export function counter_get():void {
    let uref = getKey(COUNT_KEY);
    if(uref ===null){
        Error.fromErrorCode(ErrorCode.GetKey).revert();
    } else{
        let uref_key  = <Key>uref;
        let value =  uref_key.read();
        if(value === null) {

        } else{
            let result = fromBytesLoad<i32>(value).unwrap();
            let typed_result = CLValue.fromI32(result);
            ret(typed_result);
        }
    }
}

export function call(): void {
    // Initialize counter
    let counter_local_key = Key.create(CLValue.fromI32(<i32>0));
    if(counter_local_key === null){

    } else {
        let key = <Key>counter_local_key;
        // Create initial named keys of the contract.
        let counter_named_keys = new Array<Pair<string,Key>>();
        counter_named_keys.push(new Pair<string,Key>(COUNT_KEY,key));

        let counter_entry_points = new EntryPoints();
        let args = new Array<Pair<String, CLType>>();

        // Create inc and get entry point
        let counter_inc_entry_point = new EntryPoint(COUNTER_INC,args,new CLType(CLTypeTag.Unit,null),new PublicAccess(),EntryPointType.Contract);
        let counter_get_entry_point = new EntryPoint(COUNTER_GET,args,new CLType(CLTypeTag.I32,null),new PublicAccess(),EntryPointType.Contract);

        // Add entry points
        counter_entry_points.addEntryPoint(counter_inc_entry_point);
        counter_entry_points.addEntryPoint(counter_get_entry_point);

        // Create locked contract
        let addContractVersionResult = newLockedContract(counter_entry_points,counter_named_keys,null,null);
        putKey(COUNTER_KEY,Key.fromHash(addContractVersionResult.contractHash)); 
    }   
}
