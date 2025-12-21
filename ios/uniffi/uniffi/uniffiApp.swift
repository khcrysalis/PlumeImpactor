//
//  uniffiApp.swift
//  uniffi
//
//  Created by samsam on 12/20/25.
//

import SwiftUI

let username_a = ""
let password_a = ""

@main
struct uniffiApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
                .onAppear { 
                    handleProviderInstall()
                }
        }
    }
}
