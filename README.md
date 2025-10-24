This project requires a GitHub Personal Access Token.
Token Instructions:
1. Create a new Personal Access Token by following the GitHub documentation: Creating a personal access token.
2. You do not need to grant any special permissions. A token with no scopes selected is fine.
3. Copy the generated token and in the root directory of this project.
4. Using the .env.template file, replace YOUR\_TOKEN\_HERE with your token.

Run Instructions:
1. Make sure you have Redis installed and running at redis://127.0.0.1/.
2. Use the command: cargo run

Test Instructions:
Use the command: cargo test

Assumptions made:
-Assumed C and C++ were to be separated repos
-Assumed we only needed to save 1 repo from each language as in Part C
-Assumed for a repo to be considered as a valid candidate of a language, it had to be the majority language used in the repo
-Assumed that if the repo had build files alongside source code, we assumed that it wasn’t a tutorial or documentation repo