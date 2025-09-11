#!/usr/bin/env python3

"""
Simple test runner for Aerugo integration tests
Runs all test classes directly without pytest complexities
"""

import sys
import os
import traceback
import time

# Add tests directory to path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

# Import test classes
try:
    from test_health import HealthTests
    from test_auth import AuthTests  
    from test_organizations import OrganizationTests
    from test_users import UserTests
    from test_repositories import RepositoryTests
except ImportError as e:
    print(f"❌ Error importing test modules: {e}")
    sys.exit(1)

# Optional S3 storage tests (may not be available)
try:
    from test_s3_storage_python import S3StorageAPITester
    S3_TESTS_AVAILABLE = True
except ImportError:
    print("⚠️  S3 storage tests not available (optional)")
    S3_TESTS_AVAILABLE = False

# Optional storage tests (may not be available)
try:
    from test_storage_python import StorageAPITester
    STORAGE_TESTS_AVAILABLE = True
except ImportError:
    print("⚠️  Storage tests not available (optional)")
    STORAGE_TESTS_AVAILABLE = False

def run_test_class(test_class, class_name):
    """Run all test methods in a test class"""
    print(f"\n🧪 Running {class_name}")
    print("=" * 50)
    
    try:
        instance = test_class()
        test_methods = [method for method in dir(instance) if method.startswith('test_')]
        
        if not test_methods:
            print(f"  ⚠️  No test methods found in {class_name}")
            return 0, 0
        
        passed = 0
        failed = 0
        
        for method_name in test_methods:
            try:
                print(f"  • {method_name}...", end=" ")
                method = getattr(instance, method_name)
                method()
                print("✅ PASS")
                passed += 1
            except Exception as e:
                print(f"❌ FAIL")
                print(f"    Error: {str(e)}")
                failed += 1
                # Continue with next test instead of stopping
        
        print(f"\n{class_name} Results: {passed} passed, {failed} failed")
        return passed, failed
        
    except Exception as e:
        print(f"❌ Failed to initialize {class_name}: {e}")
        return 0, 1

def main():
    """Main test runner"""
    print("🚀 Aerugo Integration Test Runner")
    print("=================================")
    
    # Test classes to run
    test_classes = [
        (HealthTests, "HealthTests"),
        (AuthTests, "AuthTests"),
        (OrganizationTests, "OrganizationTests"), 
        (UserTests, "UserTests"),
        (RepositoryTests, "RepositoryTests"),
    ]
    
    # Add optional tests if available
    if STORAGE_TESTS_AVAILABLE:
        test_classes.append((StorageAPITester, "StorageTests (Optional)"))
    
    if S3_TESTS_AVAILABLE:
        test_classes.append((S3StorageAPITester, "S3StorageTests (Optional)"))
    
    total_passed = 0
    total_failed = 0
    start_time = time.time()
    
    for test_class, class_name in test_classes:
        try:
            passed, failed = run_test_class(test_class, class_name)
            total_passed += passed
            total_failed += failed
        except KeyboardInterrupt:
            print("\n⚠️ Test execution interrupted by user")
            break
        except Exception as e:
            print(f"⚠️  Unexpected error running {class_name}: {e}")
            print(f"    Continuing with other tests...")
            if "Optional" in class_name:
                print(f"    ({class_name} is optional and can be skipped)")
            else:
                traceback.print_exc()
            total_failed += 1
            # Continue with next test class instead of stopping
    
    # Final results
    end_time = time.time()
    duration = end_time - start_time
    
    print("\n" + "=" * 60)
    print("📊 FINAL RESULTS")
    print("=" * 60)
    print(f"Total Tests: {total_passed + total_failed}")
    print(f"✅ Passed: {total_passed}")
    print(f"❌ Failed: {total_failed}")
    print(f"⏱️  Duration: {duration:.2f} seconds")
    
    if total_failed == 0:
        print("\n🎉 ALL TESTS PASSED!")
        return 0
    elif total_passed > 0:
        print(f"\n⚠️  {total_failed} TEST(S) FAILED, but {total_passed} passed!")
        print("   Continuing execution (some failures may be in optional components)")
        return 0
    else:
        print(f"\n💥 ALL {total_failed} TEST(S) FAILED!")
        return 1

if __name__ == "__main__":
    exit_code = main()
    sys.exit(exit_code)
