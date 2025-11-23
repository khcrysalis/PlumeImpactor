//
//  gestalt.h
//  gestalt
//
//  Created by samsam on 11/23/25.
//

#ifndef GESTALT_H_
#define GESTALT_H_

#include <CoreFoundation/CoreFoundation.h>
	
CFPropertyListRef MGCopyAnswer(CFStringRef property);
static const CFStringRef kMGUniqueDeviceID = CFSTR("UniqueDeviceID");

#endif /* GESTALT_H_ */
