![PostgreChain Logo](/doc/postgrechain_logo_sm.png)

# Custom Solana Program Accounts PostgreSQL Extension: postgreChain

postgreChain is designed to integrate seamlessly with Solana blockchain data, enabling users to query program accounts directly from a PostgreSQL database. It leverages the power of Rust and the pgx framework to provide a high-performance, reliable, and easy-to-use solution for developers and analysts working with Solana blockchain data.

## Features

- Direct Blockchain Access: Fetch program account details directly from the Solana blockchain into your PostgreSQL database.
- High Performance: Written in Rust and optimized for efficiency, this extension offers fast query execution and minimal latency.
- Easy Integration: Seamlessly integrates with existing PostgreSQL databases, allowing for the addition of blockchain data to your relational datasets.
- Flexible Querying: Supports a range of queries, from simple account lookups to complex analytical queries, to meet diverse data analysis needs.
- Secure: Implements best practices to ensure secure data access and transactions.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

If you encounter any issues or have questions about using the extension, please open an issue on the GitHub repository.

---

# Installing a Custom PostgreSQL Extension with Docker for PostgreSQL 13

This guide provides a step-by-step walkthrough for installing a custom PostgreSQL extension, compiled into a `.so` (shared object) file using Rust and the `pgx` library, into a PostgreSQL 13 database running inside a Docker container.

## Prerequisites

- Docker installed on your machine.
- Rust and the `pgrx` framework installed for developing PostgreSQL extensions.
- A PostgreSQL extension project developed with `pgrx`.
- PostgreSQL client tools installed locally (optional, for testing purposes).

## Step 1: Compile Your Extension

Before building your Docker image, you need to compile your extension using `pgrx`. This will produce the `.so`, `.sql`, and `.control` files needed to install your extension in PostgreSQL.

1. In your terminal, navigate to your extension project directory.
2. Run the following `pgrx` command:

```bash

   cargo pgrx run pg13

```

This command compiles your extension and runs a temporary PostgreSQL 13 instance with your extension installed, primarily for testing purposes. However, we're interested in the compiled extension files.

Once pgx has finished, it will output the paths to the .so, .sql, and .control files. Note these paths as you will need to copy these files into your Docker container.

## Step 2: Prepare Your Dockerfile

Create a Dockerfile in your project directory. This Dockerfile will be based on the official PostgreSQL 13 Docker image and will include your custom extension.

```dockerfile

# Use the official PostgreSQL 13 image as the base
FROM postgres:13

# Set the working directory in the container
WORKDIR /usr/src/my_extension

# Copy the compiled extension (.so file) and the associated .control and .sql files
# Replace the paths with the actual paths to your compiled files
COPY /path/to/your_extension.so /usr/lib/postgresql/13/lib/
COPY /path/to/your_extension.control /usr/share/postgresql/13/extension/
COPY /path/to/your_extension--0.1.sql /usr/share/postgresql/13/extension/


```

Make sure to replace /path/to/... with the actual paths to your compiled extension files noted in Step 1.

## Step 3: Build Your Docker Image

With your Dockerfile ready, build your Docker image using the following command:

```bash

docker build -t my_custom_postgres_13 .

```

This command builds a new Docker image named my_custom_postgres_13 containing your extension.

## Step 4: Run Your PostgreSQL 13 Container

Now, run a container from your custom image:

```bash
docker run --name my_postgres_13_container -e POSTGRES_PASSWORD=mysecretpassword -d my_custom_postgres_13

```

Replace mysecretpassword with a secure password of your choice.

---

# pc_balance

The `pc_balance` function is a PostgreSQL extension that queries the balance of a specified Solana wallet address. It leverages the Solana blockchain's capabilities to fetch the current balance in lamports and is designed for integration into SQL-based applications and services.

## Parameters

- **WalletPublicKeyBase58**: The public key of the Solana wallet whose balance you want to query, encoded in Base58.
- **NetworkIdentifier**: A string indicating the Solana network to use for the query. Valid options are `mainnet`, `devnet`, `testnet`, and `localhost`.

## Usage

To query the balance of a Solana wallet, invoke the `pc_balance` function within a SQL `SELECT` statement as follows:

```sql

SELECT pc_balance('WalletPublicKeyBase58', 'NetworkIdentifier');

```

# pc_create_wallet

The `pg_create_wallet` function is a PostgreSQL extension that generates a new Solana wallet, including both the public and secret keys. This function is designed to be used directly within SQL queries to easily generate wallet addresses and their corresponding secret keys.

## Usage

To use the `pg_create_wallet` function, simply invoke it within a SQL `SELECT` statement as follows:

```sql

SELECT public_key, secret_key FROM pg_create_wallet();


```

# pc_transfer

The `pc_transfer` function is a PostgreSQL extension designed to perform SOL transfers within the Solana blockchain. It utilizes the `sol_transfer` Rust function to carry out the transactions. This function is especially useful for integrating Solana blockchain transactions directly into SQL-based applications and services.

## Parameters

- **SenderPublicKeyBase58**: The public key of the sender's Solana wallet, encoded in Base58.
- **SenderSecretKeyBase58**: The secret key of the sender's Solana wallet, encoded in Base58. **Handle with care.**
- **RecipientPublicKeyBase58**: The public key of the recipient's Solana wallet, encoded in Base58.
- **AmountInSOL**: The amount of SOL to be transferred, specified in whole SOL (not lamports).
- **NetworkIdentifier**: A string indicating the Solana network to use (`mainnet`, `devnet`, `testnet`, `localhost`).

## Usage

To execute a SOL transfer using the `pc_transfer` function, invoke it within a SQL `SELECT` statement as follows:

```sql

SELECT pc_transfer(
    'SenderPublicKeyBase58',
    'SenderSecretKeyBase58',
    'RecipientPublicKeyBase58',
    AmountInSOL,
    'NetworkIdentifier'
);

```

# pc_token_account_balance

The `pc_token_account_balance` function is a PostgreSQL extension designed to query the balance of an SPL Token account on the Solana blockchain. This function enables SQL-based applications and services to directly access SPL Token account balances.

## Parameters

- **TokenAccountPublicKeyBase58**: The public key of the SPL Token account whose balance you want to query, encoded in Base58.
- **NetworkIdentifier**: A string indicating the Solana network to use for the query. Valid options are `mainnet`, `devnet`, `testnet`, and `localhost`.

## Usage

To query the balance of an SPL Token account, invoke the `pc_token_account_balance` function within a SQL `SELECT` statement as follows:

```sql

SELECT pc_token_account_balance('TokenAccountPublicKeyBase58', 'NetworkIdentifier');

```

# pc_get_program_accounts

This PostgreSQL extension provides a custom function `pc_get_program_accounts` to query Solana program accounts directly from your PostgreSQL database. It connects to the Solana blockchain, fetches accounts associated with a specified program ID, and returns their details for analysis within PostgreSQL.

## Function Overview

The `pc_get_program_accounts` function fetches account details for a given program ID from the Solana blockchain. It uses the specified network (e.g., mainnet, testnet) to perform the query.

### Arguments

- `program_id`: The ID of the program whose accounts you want to query. This should be a valid Solana program ID in string format.
- `network_str`: The network to query against. This should be a string representing the network name, such as "mainnet", "testnet", or "devnet".

### Return Value

Returns a table iterator containing the following columns for each account associated with the specified program ID:

- `public_key`: The public key of the account (String).
- `data_len`: The length of the account's data field (i64).
- `lamports`: The amount of lamports in the account (i64).
- `rent_epoch`: The rent epoch of the account (i64).
- `executable`: A boolean flag indicating whether the account is executable.

### Error Handling

If an error occurs (e.g., network issues, invalid program ID), the function will return an error message as a string.

## Usage Example

To use the `pc_get_program_accounts` function in your SQL queries, follow the example below:

```sql

SELECT * FROM pc_get_program_accounts('4Nd1mBQtrMJVYVfKf2PJy9NZUZdTAsp7D4xWLs4gDB4T', 'testnet');

```
