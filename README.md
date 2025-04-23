# gup

**gup** (**Git User Profile**) is a CLI that helps you manage multiple Git user profiles easily on a single machine. 
It uses `SSH` to connect to your GitHub account.

## Features

- Create and store multiple Git profiles
- Clone Git repositories with a selected profile (`gup clone`)
- Show current Git user config (`gup current`)
- View, edit, and delete saved profiles
- Show current Git user config (`gup current`)
- Test SSH connection of a profile

## Installation

- **Linux/macOS/WSL**

```bash
curl -sSL https://raw.githubusercontent.com/heryfitiavana22/gup/main/scripts/install.sh | bash
```

- **Windows**

```bash
curl -o "%TEMP%\install.bat" https://raw.githubusercontent.com/heryfitiavana22/gup/main/scripts/install.bat && "%TEMP%\install.bat"
```

## Usage

| Command                                       | Description                                           |
| --------------------------------------------- | ----------------------------------------------------- |
| [add](#add-a-new-profile)                     | Create a new Git profile                              |
| [use](#use-a-profile-set-git-user-config)     | Apply a profile to the current Git config             |
| [clone](#clone-with-profile-specific-via-ssh) | Clone a repo using a selected profile                 |
| [test](#test-a-profiles-ssh-connection)       | Check if the profile's SSH key is valid               |
| [copy](#copy-ssh-public-key-to-the-clipboard) | Copy the SSH public key of a profile to the clipboard |
| [current](#show-current-active-git-user)      | Show the currently active Git user                    |
| [list](#list-all-saved-profiles)              | List all saved profiles                               |
| [show](#show-details-of-one-profile)          | Show details of a specific profile                    |
| [edit](#edit-a-profile)                       | Modify an existing profile                            |
| [remove](#remove-a-profile)                   | Delete a saved profile                                |

### Add a new profile

- Example

```bash
# Add a new Git user profile
gup add
```

### Use a profile (set Git user config)

- Example

```bash
# specific profile
gup use pro

# open selected profile created
gup use
```

- Command help

```bash
# Set the current Git user profile (local or global)

Usage: gup use [OPTIONS] [USERNAME]

Arguments:
  [USERNAME]  Git username to use

Options:
  -g, --global  Apply globally (default: local)
```

### Clone with profile-specific via SSH

- Example

```bash
# specific profile
gup clone git@github.com:example/repo.git -u prod

# open selected profile created
gup clone git@github.com:example/repo.git
```

- Command help

```bash
# Clone a Git repo using a specific profile

Usage: gup clone [OPTIONS] <REPO_URL>

Arguments:
  <REPO_URL>  The git URL to clone

Options:
  -u, --username <USERNAME>  Git username to use
```

### Test a profile's SSH connection

- Example

```bash
# specific profile
gup test pro

# open selected profile created
gup test
```

- Command help

```bash
# Set the current Git user profile (local or global)

Usage: gup test [USERNAME]

Arguments:
  [USERNAME] Git username to test ssh connection
```

### Copy SSH public key to the clipboard

- Example

```bash
# specific profile
gup copy pro

# open selected profile created
gup copy
```

- Command help

```bash
# Copy the SSH public key of a profile to the clipboard

Usage: gup copy [USERNAME]

Arguments:
  [USERNAME]  Git username to copy SSH public key
```

### Show current active Git user

- Example

```bash
# Show the currently active Git profile
gup current
```

### Edit a profile

- Example

```bash
# specific profile
gup edit pro

# open selected profile created
gup edit
```

- Command help

```bash
# Edit an existing profile

Usage: gup copy [USERNAME]

Arguments:
  [USERNAME]  Git username to edit
```

### List all saved profiles

- Example

```bash
# List all available profiles
gup list
```

### Show details of one profile

- Example

```bash
# specific profile
gup show pro

# open selected profile created
gup show
```

- Command help

```bash
# Show details of a specific profile

Usage: gup show [USERNAME]

Arguments:
  [USERNAME]  Git username to display
```

### Remove a profile

- Example

```bash
# specific profile
gup remove pro

# open selected profile created
gup remove
```

- Command help

```bash
# Remove a saved profile

Usage: gup remove [USERNAME]

Arguments:
  [USERNAME]  Git username to remove
```
