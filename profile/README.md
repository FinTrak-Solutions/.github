```text
 ______  __  __   __  ______  ______  ______  ______  __  __    
/\  ___\/\ \/\ "-.\ \/\__  _\/\  == \/\  __ \/\  ___\/\ \/ /    
\ \  __\\ \ \ \ \-.  \/_/\ \/\ \  __<\ \  __ \ \ \___\ \  _"-.  
 \ \_\   \ \_\ \_\\"\_\ \ \_\ \ \_\ \_\ \_\ \_\ \_____\ \_\ \_\ 
  \/_/    \/_/\/_/ \/_/  \/_/  \/_/ /_/\/_/\/_/\/_____/\/_/\/_/
```

### Welcome to FinTrak Solutions 👋
> Rust native TUI financial tracker with persistent storage and reporting functions.

# Our Crew
| Name    | Student Number | Email |
| -------- | ------- | ------- |
| Ke Li  | 1005842554 | |
| Sarah Tang | 1002397079 | jinzhuo.tang@mail.utoronto.ca |
| Ellen Pan  | 1002159353 | |

# Motivation
In today’s fast-paced world, financial management is crucial for individuals seeking to maintain control over their income and expenses. Although there are various finance related apis and accounting tools available in existing Rust crates, there lacks a centralized tool for users to organize the information, track their spending habits, as well as analyzing their personal expenses. To address this gap, we proposed developing a robust, user-friendly personal finance tracker FIRE using Rust, tailored for users looking for a simplified yet powerful tool to handle their financial management effectively.

# Objectives
We aim to deliver a versatile text user interface(TUI) solution designed to empower users with seamless tracking of income and expenses across customizable categories, while managing multiple account types upon users’ needs. The tool ensures accurate financial data logging and delivers a smooth experience for users seeking insights into their spending, saving, and investment behavior. Featuring an intuitive, no-frills interface, this utility enables users to optimize their financial strategies and decision-making, to reach long-term financial goals and, eventually, achieve FIRE (Financial Independence, Retire Early).

Based on our aim, we have two key objectives in this project:

**:moneybag:Intuitive Budgeting Tools:moneybag:**

Users can access easy-to-use text user interface to set spending limits in categories like housing, utilities, and entertainment, as well as income expectations, such as salary or bonuses. By allowing budgets to be managed daily, weekly, monthly, or yearly, the tool lets users adjust tracking to fit their personal needs. With real-time updates on spending, users can continuously monitor their progress and stay aligned with their financial goals.

**:ledger:Comprehensive Financial Reports:ledger:**

Users can access customizable reports that provide clear insights into their spending and saving habits. These reports are available in different views—such as account-based, income-focused, or expense-oriented—and can be adjusted by time frame, ranging from daily to yearly. Additionally, the tool offers budget status summaries (below target, on target, or exceeded) to show users how well they’re meeting their financial goals.

# Features
In this section, we would explain the details about the features of our personal finance tracker to support the above objectives. 

## User Authentication
The tool supports user authentication for access control and account interactions. Specifically:
* When the user uses the tool for the first time, he/she would be prompted to enter a username and a password.
* Usernames must be unique. The tool would provide error messages in case of a collision in usernames.
* On subsequent tool usage, the tool would authenticate the user through the username and password.

## Account Management
There are two types of functionalities relevant to account management:
* The tool allows users to add, delete, and rename different accounts. 
   * The account names should be unique for the same user. The tool would provide error messages in case of a collision in names when adding or renaming accounts.
* The tool supports multiple types of accounts for each user. The types are defined in two levels:
  1. On the basic level, the accounts are divided into debit and credit accounts.
  2. On the finer-grind level, the user could customize the account names based on their own needs.
 
## Budget Management
The tool allows users to set and manage budgets through **categories**. We allow categories to be managed by users, which means the users could:
* Create/delete/update categories of different types, names, and budgets.
* Get transactions in different categories.

Specifically, when it comes to budgets, the users are able to:
* Set numerical limits for expense categories.
* Set numerical expectations for income categories.
* Set frequency for budgets: daily/weekly/monthly/yearly.
* Modify previously set budgets.

## Transaction Management
The tool supports users to log their transactions and categorize them. More specifically:
* The user can classify a transaction with a category
* Optionally, the user can specify notes with each transaction.
* The user can modify a logged transaction’s amount, involved accounts, categories, and notes if applicable.
For each logged transaction, the tool would provide a transaction_id to the user.

## Analysis and Reporting
TBD

# User Guide
To help users better navitage through our tool, we would introduce how to explore our tools in this section.

## Authentication Tab
TBD

## Account Management Tab
TBD

## Category and Budget Management Tab
TBD

## Report Tab
TBD

# Reproducibility Guide
## Perequisites
In order to use our financial tracker, we assume the following libraries are available in the environment:
1. PostgresSQL 17 (Downloading and installation available at [here](https://www.postgresql.org/download/))

During the setup, please do not change the username and keep the main password you set.
The rest of this guide assumes default database username `postgres` with password `123456`.

2. Create a database with Postgres for our financial tracker.

    1. Use `psql -U postgres` to enter PostgresSQL Prompt.
    2. Within the prompt, run the following command: `CREATE DATABASE financial_tracker_db;`
    3. Verify the new database is successfully created by `\l` in the PostgresSQL Prompt
    4. Quit the PostgresSQL Prompt with `\q`

3. Diesel CLI tool (Downloading and installation guide available at [here](https://diesel.rs/guides/getting-started))

For MacOS users:
```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
```
For Windows users:
```
powershell -c "irm https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.ps1 | iex"
```

## Setting up Backend Server
1. Sync our backend repository at: https://github.com/FinTrak-Solutions/Backend.git
2. Inside Backend/backend, create a file named `.env`, with the following:
```
DATABASE_URL=postgres://postgres:123456@localhost/financial_tracker_db
```
3. Modify the `Backend/backend/disel.toml`: please replace the `dir` in migrations_directory on line 8 with you own local git repository.
4. Inside Backedn/backend, run `diesel imgration run`. You should see the migrations within backend/backend/migrations run in sequence.
5. Inside Backend/backend, run `cargo clean`. (Note: this step is necessary for this known [issue](https://stackoverflow.com/questions/70313347/ld-library-not-found-for-lpq-when-build-rust-in-macos) with Diesel)
6. Inside Backend/backend, run `cargo run`.

Your local fintrak server should be ready and running!

## Setting up the TUI Client
TBD

## ALL SET! :ship:

# Individual Contribution
We divided our work into four different categories: database setup, TUI client developement, backend server developement, and final report. Note that final report is not the only documentation we maintain, rather the backend API and frontend user guides were updated accordingly as we developed our project.
<table><thead>
  <tr>
    <th>Task</th>
    <th>Assignee</th>
    <th>Contributor</th>
  </tr></thead>
<tbody>
  <tr>
    <td colspan="3"><b>Database Setup</b></td>
  </tr>
  <tr>
    <td>Design Table Schemas</td>
    <td>Sarah Tang</td>
    <td>Ke Li, Ellen Pan</td>
  </tr>
  <tr>
    <td>Create Diesel Tables and Migrations</td>
    <td>Ke Li</td>
    <td>Sarah Tang, Ellen Pan</td>
  </tr>
  <tr>
    <td>Connect Database in Rocket Backend</td>
    <td>Ke Li</td>
    <td>/</td>
  </tr>
  <tr>
    <td>Verify Database Functionalities</td>
    <td>Ke Li</td>
    <td>/</td>
  </tr>
  <tr>
    <td colspan="3"><b>Backend Server Developement</b></td>
  </tr>
  <tr>
    <td>Rocket Framework Setup and Modularization</td>
    <td>Ke Li</td>
    <td>/</td>
  </tr>
  <tr>
    <td>Authentication</td>
    <td>Ke Li</td>
    <td>/</td>
  </tr>
  <tr>
    <td>Account Create/Delete/Update/Summary Routes Implementation</td>
    <td>Ke Li</td>
    <td>/</td>
  </tr>
  <tr>
    <td>Category Create/Delete/Update/Summary Routes Implementation</td>
    <td>Sarah Tang</td>
    <td>/</td>
  </tr>
  <tr>
    <td>Transaction Create/Delete/Update/Details Routes Implementation</td>
    <td>Sarah Tang</td>
    <td>/</td>
  </tr>
  <tr>
    <td>Report Related Routes Implementation</td>
    <td>Sarah Tang</td>
    <td></td>
  </tr>
  <tr>
    <td colspan="3"><b>TUI Client Development</b></td>
  </tr>
  <tr>
    <td>Ratatui Framework Setup and Modularization</td>
    <td>Ke Li</td>
    <td>Ellen Pan</td>
  </tr>
  <tr>
    <td>Authentication</td>
    <td>Ke Li</td>
    <td></td>
  </tr>
  <tr>
    <td>Account Summary Page</td>
    <td>Ellen Pan</td>
    <td></td>
  </tr>
  <tr>
    <td>Add/Delete/Update Account Actions</td>
    <td>Ellen Pan</td>
    <td></td>
  </tr>
  <tr>
    <td>Add/Delete/Update Category Actions</td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <td>Category Summary Page</td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <td>Add/Delete/Update Transaction Actions</td>
    <td>Ke Li</td>
    <td></td>
  </tr>
  <tr>
    <td>Report Page</td>
    <td>Sarah Tang</td>
    <td></td>
  </tr>
  <tr>
    <td colspan="3">Final Report</td>
  </tr>
  <tr>
    <td>Motivations and Objectives</td>
    <td>Ellen Pan</td>
    <td>Sarah Tang</td>
  </tr>
  <tr>
    <td>Key Features</td>
    <td>Sarah Tang</td>
    <td>/</td>
  </tr>
  <tr>
    <td>User Guide</td>
    <td></td>
    <td>Sarah Tang</td>
  </tr>
  <tr>
    <td>Reproducibility Guide</td>
    <td>Sarah Tang</td>
    <td>Ke Li, Ellen Pan</td>
  </tr>
  <tr>
    <td>Lessons Learned and Conclusion</td>
    <td>Sarah Tang</td>
    <td></td>
  </tr>
</tbody></table>

# Lessons Learned and Conclusion
It was both a great learning experience as well as a fun journey for us to develop this Rust based financial tracker. We built our knowledge in the Rust full stack world: from the Rocket backend server to the Ratatui frontend client, from Diesel interface of database to how to keep track of our documentation while working in paralle. We hope you enjoy fiddling with our financial tracker, and it would help you manage your financial situation in a clear and friendly way.

Before we close this report and bid you farewell, we would like to add a few lessons learned in our project, and hopefully they would be beneficial to any future readers:

**Establish Clear Milestones** 
One of the feedbacks that we received for our proposals was the lack of details in work distribution. We had since restructured our milestones to be more detailed, broken down the bigger goals into smaller components, and assigned clear team members to work on the items. Clear milestones helped us track and measure our progress, estimate the remaining wordload, and adjust our schedules accordingly.

**Regular Status Sync Meetings**
We held status sync meetings regularly throughout the term, with the frequencies adjusted based on due dates. These meetings help us keep our project on track, as discussion and collaboration is unavoidable for a project like this. For example, we have status sync meetings to review and settle the database schemas, to confirm environment setups, and to go over the development status and action items required for either frontend or backend. These meetings help us align our understandings and our role in implementing the features we wanted.

**Keep Documentations Up-to-Date**
It is challenging to work on a coding project in parallel, while different parts our system depend on each other (e.g. frontend needs to communicate with the backend somehow, and the request and response formats need to be in-sync to perform any useful testing and verification on the functionalities). Since we could not have status sync meetings every day, we found it very beneficial to keep documentations on both TUI end and Rocket server end with each incremental commits. The documented APIs facilitated the flexible communication among team members while catching any out-of-sync behaviours in a timely manner. 
