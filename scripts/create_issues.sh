#!/bin/bash

# Script to help create GitHub issues from the implementation plan
# Usage: ./create_issues.sh

echo "Aerugo Implementation Issues Creation Helper"
echo "==========================================="
echo ""
echo "This script will help you create GitHub issues for the Aerugo implementation plan."
echo ""
echo "Before running this script, ensure you have:"
echo "1. GitHub CLI (gh) installed and authenticated"
echo "2. Proper repository permissions to create issues"
echo ""

read -p "Do you want to create all 30 implementation issues? (y/n): " -n 1 -r
echo
if [[ ! $REPO =~ ^[Yy]$ ]]
then
    echo "Aborted."
    exit 1
fi

# Repository (should be current repo)
REPO=$(git remote get-url origin | sed 's/.*github.com[:/]\(.*\)\.git/\1/')
echo "Creating issues in repository: $REPO"
echo ""

# Array of issue data (title, labels, milestone)
declare -a issues=(
    "Initialize Rust Project Structure|setup,foundation,critical|Phase 1"
    "Configuration Management System|config,foundation,high|Phase 1"
    "Error Handling and Logging System|error-handling,logging,foundation,high|Phase 1"
    "Database Schema Design and Migrations|database,schema,migrations,critical|Phase 2"
    "Database Models and Query Layer|database,models,high|Phase 2"
    "Storage Backend Abstraction|storage,abstraction,high|Phase 3"
    "S3-Compatible Storage Implementation|storage,s3,high|Phase 3"
    "Storage Unit Tests|testing,unit-tests,storage,medium|Phase 3"
    "JWT Token Management|auth,jwt,security,high|Phase 4"
    "Permission System|auth,permissions,security,high|Phase 4"
    "Authentication Middleware|auth,middleware,api,medium|Phase 4"
    "Authentication Unit Tests|testing,unit-tests,auth,medium|Phase 4"
    "Redis Cache Implementation|cache,redis,performance,medium|Phase 5"
    "Cache Unit Tests|testing,unit-tests,cache,low|Phase 5"
    "Registry API Foundation|api,registry,docker,critical|Phase 6"
    "Blob Operations API|api,registry,blobs,critical|Phase 6"
    "Manifest Operations API|api,registry,manifests,critical|Phase 6"
    "Repository Catalog API|api,registry,catalog,medium|Phase 6"
    "Registry API Unit Tests|testing,unit-tests,api,registry,high|Phase 6"
    "Management API Foundation|api,management,json,high|Phase 7"
    "User Management API|api,management,users,high|Phase 7"
    "Organization Management API|api,management,organizations,medium|Phase 7"
    "Repository Management API|api,management,repositories,medium|Phase 7"
    "Management API Unit Tests|testing,unit-tests,api,management,high|Phase 7"
    "API Integration Tests|testing,integration,e2e,high|Phase 8"
    "Docker Client Integration Tests|testing,integration,docker,high|Phase 8"
    "Performance and Load Testing|testing,performance,benchmarks,medium|Phase 8"
    "GitHub Actions CI Pipeline|ci-cd,automation,github-actions,high|Phase 9"
    "Docker and Deployment|docker,deployment,ops,medium|Phase 9"
    "Monitoring and Observability|monitoring,metrics,observability,medium|Phase 9"
)

# Function to create a single issue
create_issue() {
    local issue_num=$1
    local title=$2
    local labels=$3
    local milestone=$4
    
    echo "Creating Issue #$issue_num: $title"
    
    # This is a placeholder - in real usage, you would extract the detailed 
    # description from IMPLEMENTATION_ISSUES.md and create the actual issue
    echo "  Labels: $labels"
    echo "  Milestone: $milestone"
    echo "  [Would create with gh cli here]"
    echo ""
    
    # Uncomment and modify this line to actually create issues:
    # gh issue create --title "[$milestone] $title" --label "$labels" --body-file <(extract_issue_body $issue_num)
}

# Create all issues
issue_num=1
for issue_data in "${issues[@]}"
do
    IFS='|' read -r title labels milestone <<< "$issue_data"
    create_issue $issue_num "$title" "$labels" "$milestone"
    ((issue_num++))
done

echo "Issue creation complete!"
echo ""
echo "Next steps:"
echo "1. Review the created issues in GitHub"
echo "2. Assign issues to team members"
echo "3. Set up project board for tracking"
echo "4. Begin implementation with Issue #1"
echo ""
echo "For detailed issue descriptions, see IMPLEMENTATION_ISSUES.md"