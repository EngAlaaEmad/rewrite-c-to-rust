#[repr(C)]
enum netCoalesceFlags {
  NET_COALESCE_NORMAL = 0,

  /**
   * retains /../ that reach above dir root (useful for FTP
   * servers in which the root of the FTP URL is not necessarily
   * the root of the FTP filesystem).
   */
  NET_COALESCE_ALLOW_RELATIVE_ROOT = 1 << 0,

  /**
   * recognizes /%2F and // as markers for the root directory
   * and handles them properly.
   */
  NET_COALESCE_DOUBLE_SLASH_IS_ROOT = 1 << 1
};


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn rust_net_CoalesceDirs(flags: netCoalesceFlags,path: *mut u8 ) {

    let fwdPtr: *mut u8 = path;
    let urlPtr: *mut u8 = path;
    let lastslash: *mut u8 = path;
    let traversal:  u32 = 0;
    let special_ftp_len:  u32 = 0;
    //let NetCoalesceDoubleSlashIsRoot: netCoalesceFlags= netCoalesceFlags::NET_COALESCE_DOUBLE_SLASH_IS_ROOT;
    
    /* Remember if this url is a special ftp one: */
    if flags & NetCoalesceDoubleSlashIsRoot {
            /* some schemes (for example ftp) have the speciality that
            the path can begin // or /%2F to mark the root of the
            servers filesystem, a simple / only marks the root relative
            to the user loging in. We remember the length of the marker */
            if *path[0..3].eq_ignore_ascii_case("/%2F"){
                special_ftp_len = 4;
            }
            else if *path[0..1].eq_ignore_ascii_case("//"){
                special_ftp_len = 2;
            }
            
    }
    /* find the last slash before # or ? */
    while *fwdPtr != '\0' && *fwdPtr != '?' && *fwdPtr != '#'{
        fwdPtr.add(1);
    }
    /* found nothing, but go back one only */
    /* if there is something to go back to */
    if fwdPtr != path && *fwdPtr == '\0' {
        fwdPtr.sub(1);
    }

    /* search the slash */
    while fwdPtr != path && *fwdPtr != '/'{
        fwdPtr.sub(1);
    }

    lastslash = fwdPtr;
    fwdPtr = path;


    /* replace all %2E or %2e with . in the path */
    /* but stop at lastchar if non null */
    while *fwdPtr != '\0' && *fwdPtr != '?' && *fwdPtr != '#' &&
    *lastslash == '\0' || fwdPtr != lastslash {
        if *fwdPtr == '%' && *(fwdPtr + 1) == '2' &&
        (*(fwdPtr.add(2)) == 'E' || *(fwdPtr.add(2)) == 'e') {
        *urlPtr += 1;
        *urlPtr = '.';
        fwdPtr.add(1);
        fwdPtr.add(1);
        } else {
        *urlPtr+= 1;
        *urlPtr = *fwdPtr;      
        }
        fwdPtr.add(1) ;
    }
    // Copy remaining stuff past the #?;
    while *fwdPtr != '\0'{
        *urlPtr += 1;
        *urlPtr = *fwdPtr;
        fwdPtr.add(1);
    }
    *urlPtr = '\0';  // terminate the url
    // start again, this time for real
    fwdPtr = path;
    urlPtr = path;

    while *fwdPtr != '\0' && *fwdPtr != '?' && *fwdPtr != '#' {
        if *fwdPtr == '/' && *(fwdPtr.add(1)) == '.' && *(fwdPtr.add(2)) == '/'{
            // remove . followed by slash
            fwdPtr.add(1);
        } else if *fwdPtr == '/' && *(fwdPtr.add(1)) == '.' && *(fwdPtr.add(2)) == '.' &&
            (*(fwdPtr.add(3)) == '/' ||
                *(fwdPtr.add(3)) == '\0' ||  // This will take care of
                *(fwdPtr.add(3)) == '?' ||   // something like foo/bar/..#sometag
                *(fwdPtr.add(3)) == '#') {
                // remove foo/..
                // reverse the urlPtr to the previous slash if possible
                // if url does not allow relative root then drop .. above root
                // otherwise retain them in the path
                if traversal > 0 || !(flags & netCoalesceFlags::NET_COALESCE_ALLOW_RELATIVE_ROOT) {
                    if urlPtr != path { urlPtr.sub(1); }   // we must be going back at least by one
                    while *urlPtr!= '/' && urlPtr!= path {
                        traversal-= 1; // count back
                        fwdPtr.add(2); // forward the fwdPtr past the ../
                        urlPtr.sub(1);
                    }
                    // if we have reached the beginning of the path
                    // while searching for the previous / and we remember
                    // that it is an url that begins with /%2F then
                    // advance urlPtr again by 3 chars because /%2F already
                    // marks the root of the path
                    if urlPtr == path && special_ftp_len > 3 {
                        urlPtr.add(1);
                        urlPtr.add(1);
                        urlPtr.add(1);
                    }
                    // special case if we have reached the end
                    // to preserve the last /
                    if *fwdPtr == '.' && *(fwdPtr.add(1)) == '\0' { urlPtr.add(1); }
                } else {
                    // there are to much /.. in this path, just copy them instead.
                    // forward the urlPtr past the /.. and copying it
        
                    // However if we remember it is an url that starts with
                    // /%2F and urlPtr just points at the "F" of "/%2F" then do
                    // not overwrite it with the /, just copy .. and move forward
                    // urlPtr.
                    if special_ftp_len > 3 && urlPtr == path + special_ftp_len - 1{
                        urlPtr.add(1);
                    }
                    else { 
                        *urlPtr+= 1;
                        *urlPtr = *fwdPtr;
                    }
                    fwdPtr.add(1);
                    *urlPtr+= 1;
                    *urlPtr = *fwdPtr;
                    fwdPtr.add(1);
                    *urlPtr += 1;
                    *urlPtr = *fwdPtr;
                }
        
                } else {
                    
                    // count the hierachie, but only if we do not have reached
                    // the root of some special urls with a special root marker
                    if *fwdPtr == '/' && *(fwdPtr.add(1)) != '.' &&
                    (special_ftp_len != 2 || *(fwdPtr.add(1)) != '/'){
                    traversal +=1;
                    }
                    // copy the url incrementaly
                    *urlPtr += 1;
                    *urlPtr = *fwdPtr;
            }
        fwdPtr.add(1);


    }

    /*
    *  Now lets remove trailing . case
    *     /foo/foo1/.   ->  /foo/foo1/
    */

    if (urlPtr > (path.add(1))) && (*(urlPtr.sub(1)) == '.') && (*(urlPtr.sub(2)) == '/') { 
       urlPtr.sub(1);
    }
    // Copy remaining stuff past the #?;
    while *fwdPtr != '\0' {
        *urlPtr+= 1;
        *urlPtr = *fwdPtr;
        fwdPtr.add(1);
    }
    *urlPtr = '\0';  // terminate the url 
 }

}
