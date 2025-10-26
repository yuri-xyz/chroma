# Create a git commit with conventional message format

This command creates a git commit of all changes following the project's commit message convention.

The commit message format follows: `type: description`

Where `type` can be:
- **feat** - A new feature
- **fix** - A bug fix
- **docs** - Documentation changes
- **refactor** - Code refactoring
- **chore** - Maintenance tasks
- **test** - Adding or updating tests
- **perf** - Performance improvements

## Instructions

1. Select the commit type from the list
2. Enter a concise description of your changes
3. The command will stage all changes and create the commit

## Command

```bash
# Step 1: Get commit type and message from user
echo "Select commit type:"
select TYPE in feat fix docs refactor chore test perf; do
  if [ -n "$TYPE" ]; then
    echo "Enter commit message description (one line):"
    read MESSAGE

    # Create commit message
    FULL_MESSAGE="$TYPE: $MESSAGE"

    # Stage all changes
    git add -A

    # Create commit
    git commit -m "$FULL_MESSAGE"
    break
  fi
done
```

This command will:
1. Prompt you to choose a commit type
2. Ask for a commit message description
3. Stage all changes with `git add -A`
4. Create the commit with the formatted message
