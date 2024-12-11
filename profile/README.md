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
* 
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

### Analysis and Reporting
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
2. Diesel CLI tool (Downloading and installation guide available at [here](https://diesel.rs/guides/getting-started))
3. TBD

## Setting up Backend Server
TBD

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
    <td></td>
    <td></td>
  </tr>
  <tr>
    <td colspan="3"><b>TUI Client Development</b>></td>
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
    <td></td>
    <td></td>
  </tr>
  <tr>
    <td>Add/Delete/Update Account Actions</td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <td>Account Summary Page</td>
    <td></td>
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
    <td></td>
    <td></td>
  </tr>
  <tr>
    <td>Report Page</td>
    <td></td>
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
    <td></td>
    <td>Sarah Tang</td>
  </tr>
  <tr>
    <td>Lessons Learned and Conclusion</td>
    <td></td>
    <td></td>
  </tr>
</tbody></table>

# Lessons Learned and Conclusion
