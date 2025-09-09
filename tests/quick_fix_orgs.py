#!/usr/bin/env python3

"""
Quick fix for organization tests - add ensure_setup to all test methods
"""

import sys
import os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

def add_setup_to_test_methods():
    """Add ensure_setup() call to all test methods"""
    
    # Read the current file
    with open('test_organizations.py', 'r') as f:
        content = f.read()
    
    # List of test methods that need ensure_setup
    test_methods = [
        'test_organization_retrieval',
        'test_organization_update', 
        'test_organization_member_management',
        'test_user_organizations',
        'test_organization_permissions',
        'test_organization_validation',
        'test_nonexistent_organization'
    ]
    
    # Add ensure_setup to each method
    for method in test_methods:
        # Find the method definition
        method_start = f'def {method}(self):'
        if method_start in content:
            # Find the first line after the method definition  
            lines = content.split('\n')
            for i, line in enumerate(lines):
                if method_start in line:
                    # Insert ensure_setup after the docstring (usually 2-3 lines down)
                    for j in range(i+1, min(i+5, len(lines))):
                        if 'self.logger.info' in lines[j] or 'response =' in lines[j] or 'self.assert' in lines[j]:
                            # Insert ensure_setup before this line
                            lines.insert(j, '        self.ensure_setup()')
                            lines.insert(j+1, '')
                            break
                    break
            
            content = '\n'.join(lines)
    
    # Write back to file
    with open('test_organizations.py', 'w') as f:
        f.write(content)
    
    print("✅ Added ensure_setup() to all organization test methods")

if __name__ == "__main__":
    add_setup_to_test_methods()
