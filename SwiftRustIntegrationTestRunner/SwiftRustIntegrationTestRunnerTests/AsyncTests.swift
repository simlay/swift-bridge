//
//  SharedStructAttributeTests.swift
//  SwiftRustIntegrationTestRunnerTests
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class AsyncTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testSwiftCallsRustAsyncFn() async throws {
        await rust_async_return_null()
    }
   
    /// Verify that we can call async Rust functions
    func testSwiftCallsRustAsyncFnReflectU8() async throws {
        let num = await rust_async_reflect_u8(123)
        XCTAssertEqual(num, 123)
    }
    
    /// Verify that we can call async Rust methods
    func testSwiftCallsRustAsyncMethodReflectU16() async throws {
        let test = TestRustAsyncSelf()

        let num = await test.reflect_u16(567)
        XCTAssertEqual(num, 567)
    }

    
    func testSwiftCallsRustAsyncFnRetStruct() async throws {
        let _: AsyncRustFnReturnStruct = await rust_async_return_struct()
    }
}
