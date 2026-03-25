#!/bin/bash
echo "# COMELEC-CTUMC System Activity Report" > CHANGELOG.md
echo "## Generated on $(date)" >> CHANGELOG.md
git log --pretty=format:"* %s (%h)" >> CHANGELOG.md
