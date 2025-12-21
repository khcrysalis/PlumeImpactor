//
//  ContentView.swift
//  UniFFI Login Example
//

import SwiftUI
import Foundation

// MARK: - Notification for 2FA request
extension Notification.Name {
    static let requestTwoFA = Notification.Name("requestTwoFA")
}

// MARK: - TwoFaHandler
final class TwoFaHandler: TwoFaCallback, @unchecked Sendable {
    private var code: String?
    private let semaphore = DispatchSemaphore(value: 0)

    func getCode() -> String {
        DispatchQueue.main.async {
            NotificationCenter.default.post(name: .requestTwoFA, object: self)
        }

        semaphore.wait()
        return code ?? ""
    }

    func submit(code: String) {
        self.code = code
        semaphore.signal()
    }
}


// MARK: - LoginViewModel
@MainActor
final class LoginViewModel: ObservableObject {
    @Published var username = username_a
    @Published var password = password_a
    @Published var twoFACode = ""
    @Published var showing2FA = false
    @Published var status = ""

    private var handler: TwoFaHandler?

    init() {
        NotificationCenter.default.addObserver(
            forName: .requestTwoFA,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            Task { @MainActor in
                self?.showing2FA = true
            }
        }
    }

    func login() {
        status = "Logging in…"

        Task {
            do {
                let handler = TwoFaHandler()
                self.handler = handler

                let account = try await startAccountLogin(
                    username: username,
                    password: password,
                    configPath: "/tmp",
                    twoFa: handler
                )
                
                print(account.username)
                print(account.adsid)
                print(account.xcodeGsToken)

                status = "Login successful"
            } catch {
                status = "Error: \(error)"
            }
        }
    }

    func submit2FA() {
        showing2FA = false
        handler?.submit(code: twoFACode)
        twoFACode = ""
    }
}

// MARK: - ContentView
struct ContentView: View {
    @StateObject private var vm = LoginViewModel()

    var body: some View {
        VStack(spacing: 16) {
            Text("Sign In")
                .font(.largeTitle)
                .bold()

            TextField("Apple ID", text: $vm.username)
                .textContentType(.username)
                .autocorrectionDisabled()
                .textFieldStyle(.roundedBorder)

            SecureField("Password", text: $vm.password)
                .textContentType(.password)
                .textFieldStyle(.roundedBorder)

            Button("Login") {
                vm.login()
            }
            .buttonStyle(.borderedProminent)
            .disabled(vm.username.isEmpty || vm.password.isEmpty)

            Text(vm.status)
                .foregroundColor(.secondary)
        }
        .padding()
        .alert("Two-Factor Authentication", isPresented: $vm.showing2FA) {
            TextField("Verification Code", text: $vm.twoFACode)

            Button("Verify") {
                vm.submit2FA()
            }

            Button("Cancel", role: .cancel) {
                vm.submit2FA()
            }
        } message: {
            Text("Enter the code sent to your device.")
        }
    }
}
