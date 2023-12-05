/*
 * Copyright (c) 2023 robustmq team 
 * 
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * 
 *     http://www.apache.org/licenses/LICENSE-2.0
 * 
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use super::*;

fn len() -> usize {
    // pkid
    2
}

pub fn read(fixed_header: FixedHeader, mut bytes: Bytes) -> Result<PubRec, Error>{
    let variable_header_index = fixed_header.fixed_header_len;
    bytes.advance(variable_header_index);
    let pkid = read_u16(&mut bytes)?;
    if fixed_header.remaining_len == 2 {
        return Ok(PubRec{
            pkid,
            reason: PubRecReason::Success,
        });
    }

    if fixed_header.remaining_len < 4 {
        return Ok(PubRec { 
            pkid,
            reason: PubRecReason::Success,
        });
    }

    let pubrec = PubRec {
        pkid,
        reason: PubRecReason::Success,
    };
    
    Ok(pubrec)
}

pub fn write(pubrec: &PubRec, buffer: &mut BytesMut) -> Result<usize, Error> {

    let len = len();
    buffer.put_u8(0x50);
    let count = write_remaining_length(buffer, len)?;
    buffer.put_u16(pubrec.pkid);
    Ok(1 + count + len)
}
