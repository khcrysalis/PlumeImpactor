//
//  main.m
//  gestalt
//
//  Created by samsam on 11/23/25.
//

#import <Foundation/Foundation.h>
#import "gestalt.h"

int main(int argc, const char * argv[]) {
	@autoreleasepool {
		CFStringRef prop = kMGUniqueDeviceID;
		CFStringRef udid = MGCopyAnswer(prop);

		if (udid) {
			NSString *udidString = (__bridge_transfer NSString *)udid;
			printf("%s\n", [udidString UTF8String]);
		}
	}
	return 0;
}
