#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn rust_net_ResolveRelativePath<'a,'b ,'c >(
    relativePath: &'a nsACString,
    basePath: &'b nsACString ,
    result: &'c mut nsACString,
) -> nsresult {

    let name: &nsAutoCString;
    let path: &nsAutoCString = basePath ;
    let mut needsDelim: bool = false;

    if !path.is_empty(){
        let mut last: u16 = path.last();
        needsDelim = !(last == '/');
    }
    let beg,end: &nsACString::const_iterator;
    relativePath.BeginReading(beg);
    relativePath.EndReading(end);
    let stop: bool = false;
    let c: char;
    
    while (!stop){
        c = if beg == end {'\0'} else { *beg };
        match c {
            '\0'|'#'|'?' => stop = true,
            '/' => {
                if name == ("..") {
                    let offset: u32 = path.len() - if needsDelim {1} else {2};
                    if offset < 0 {
                        let ns_error: nsresult::NS_ERROR_MALFORMED_URI;
                        return nserror;
                    }
                    let pos: u32 = path.
                }
                
            }



        }
        


    }

    
    





}