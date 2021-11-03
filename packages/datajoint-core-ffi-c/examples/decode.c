#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <datajoint-core-ffi-c.h>

void print_typed_data(const void *data, size_t data_size, NativeTypeEnum data_type)
{
    switch (data_type)
    {
    case NativeTypeEnum_None:
        printf("None\n");
        break;
    case NativeTypeEnum_Null:
        printf("null\n");
        break;
    case NativeTypeEnum_Bool:
        printf("%s\n", *(int8_t *)data ? "true" : "false");
        break;
    case NativeTypeEnum_Int8:
        printf("%d\n", *(int8_t *)data);
        break;
    case NativeTypeEnum_UInt8:
        printf("%u\n", *(uint8_t *)data);
        break;
    case NativeTypeEnum_Int16:
        printf("%d\n", *(int16_t *)data);
        break;
    case NativeTypeEnum_UInt16:
        printf("%u\n", *(uint16_t *)data);
        break;
    case NativeTypeEnum_Int32:
        printf("%d\n", *(int32_t *)data);
        break;
    case NativeTypeEnum_UInt32:
        printf("%u\n", *(uint32_t *)data);
        break;
    case NativeTypeEnum_Int64:
        printf("%lld\n", *(int64_t *)data);
        break;
    case NativeTypeEnum_UInt64:
        printf("%llu\n", *(uint64_t *)data);
        break;
    case NativeTypeEnum_Float32:
        printf("%f\n", *(float *)data);
        break;
    case NativeTypeEnum_Float64:
        printf("%f\n", *(double *)data);
        break;
    case NativeTypeEnum_String:
        printf("\"%s\"\n", (const char *)data);
        break;
    case NativeTypeEnum_Bytes:
    {
        for (size_t i = 0; i < data_size; ++i)
        {
            printf("%x", ((unsigned char *)data)[i]);
        }
    }
    break;
    }
}

void generically_print_cursor_results(Cursor *cursor)
{
    if (!cursor)
    {
        return;
    }

    // Read results one row at a time.
    TableRow *next_row = NULL;
    AllocatedDecodedValue *value = allocated_decoded_value_new();
    size_t row_num = 0;
    for (;;)
    {
        ++row_num;
        // Get next row.
        int err = cursor_next(cursor, &next_row);
        if (err == ErrorCode_Success)
        {
            printf("Row %zu\n", row_num);

            TableColumnRef *columns = NULL;
            size_t columns_size = 0;
            size_t offset_amount = 0;
            if (table_row_columns(next_row, &columns, &columns_size) != ErrorCode_Success)
            {
                printf("Failed to get columns for current row: %s\n", datajoint_core_get_last_error_message());
                break;
            }
            for (size_t i = 0; i < columns_size; ++i)
            {
                TableColumnRef *next_column = table_row_columns_advance(columns, i);
                if (next_column == NULL)
                {
                    printf("Invalid column pointer\n");
                    table_row_columns_free(columns, columns_size);
                    continue;
                }
                const char *column_name = table_column_ref_name(next_column);
                if (table_row_decode_to_allocation(next_row, next_column, value) != ErrorCode_Success)
                {
                    printf("Failed to decode column \"%s\": %s\n", column_name, datajoint_core_get_last_error_message());
                    continue;
                }

                NativeTypeEnum value_type = allocated_decoded_value_type(value);
                const void *value_data = allocated_decoded_value_data(value);
                size_t value_size = allocated_decoded_value_size(value);

                printf("%s: ", column_name);
                print_typed_data(value_data, value_size, value_type);
            }
            printf("\n");
            table_row_columns_free(columns, columns_size);
        }
        else if (err == ErrorCode_NoMoreRows)
        {
            break;
        }
        else
        {
            printf("Failed to fetch next row for query: %s\n", datajoint_core_get_last_error_message());
            break;
        }
    }
    table_row_free(next_row);
    next_row = NULL;

    allocated_decoded_value_free(value);
    value = NULL;
}

void generically_print_cursor_results_with_buffer(Cursor *cursor)
{
    if (!cursor)
    {
        return;
    }

    TableRow *next_row = NULL;
    size_t buffer_size = 100;
    void *buffer = malloc(buffer_size);
    size_t row_num = 0;
    for (;;)
    {
        ++row_num;
        // Get next row.
        int err = cursor_next(cursor, &next_row);
        if (err == ErrorCode_Success)
        {
            printf("Row %zu\n", row_num);

            TableColumnRef *columns = NULL;
            size_t columns_size = 0;
            size_t offset_amount = 0;
            if (table_row_columns(next_row, &columns, &columns_size) != ErrorCode_Success)
            {
                printf("Failed to get columns for current row: %s\n", datajoint_core_get_last_error_message());
                break;
            }
            for (size_t i = 0; i < columns_size; ++i)
            {
                TableColumnRef *next_column = table_row_columns_advance(columns, i);
                if (next_column == NULL)
                {
                    printf("Invalid column pointer\n");
                    table_row_columns_free(columns, columns_size);
                    continue;
                }
                const char *column_name = table_column_ref_name(next_column);
                size_t output_size = 0;
                NativeTypeEnum output_type = NativeTypeEnum_None;
                if (table_row_decode_to_buffer(next_row, next_column, buffer, buffer_size, &output_size, &output_type) != ErrorCode_Success)
                {
                    printf("Failed to decode column \"%s\": %s\n", column_name, datajoint_core_get_last_error_message());
                    continue;
                }

                printf("%s: ", column_name);
                print_typed_data(buffer, output_size, output_type);
            }
            printf("\n");
            table_row_columns_free(columns, columns_size);
        }
        else if (err == ErrorCode_NoMoreRows)
        {
            break;
        }
        else
        {
            printf("Failed to fetch next row for query: %s\n", datajoint_core_get_last_error_message());
            break;
        }
    }
    table_row_free(next_row);
    next_row = NULL;

    free(buffer);
    buffer = NULL;
}

int main()
{
    // Initialize settings.
    ConnectionSettings *settings = connection_settings_new();
    connection_settings_set_database_type(settings, DatabaseType_MySql);
    connection_settings_set_username(settings, "username");
    connection_settings_set_password(settings, "password");
    connection_settings_set_hostname(settings, "tutorial-db.datajoint.io");
    connection_settings_set_port(settings, 3306);
    connection_settings_set_database_name(settings, "username_tutorial");

    // Create and establish connection.
    Connection *my_conn = connection_new(settings);
    settings = NULL;
    if (connection_connect(my_conn) != ErrorCode_Success)
    {
        printf("Failed to connect to database: %s\n", datajoint_core_get_last_error_message());
        connection_free(my_conn);
        return 1;
    }

    // Read query results with cursor.
    Cursor *cursor = NULL;
    if (connection_fetch_query(my_conn, "select * from mouse;", NULL, &cursor) != ErrorCode_Success)
    {
        printf("Failed to create cursor for query: %s\n", datajoint_core_get_last_error_message());
        connection_free(my_conn);
        return 1;
    }
    generically_print_cursor_results(cursor);
    cursor_free(cursor);
    cursor = NULL;

    // Switch to Postgres.
    settings = connection_get_settings(my_conn);
    connection_settings_set_database_type(settings, DatabaseType_Postgres);
    connection_settings_set_username(settings, "username");
    connection_settings_set_password(settings, "password");
    connection_settings_set_hostname(settings, "postgres url here");
    connection_settings_set_port(settings, 5432);
    connection_settings_set_database_name(settings, "database_name");

    // Reconnect with updated settings.
    if (connection_reconnect(my_conn) != ErrorCode_Success)
    {
        printf("Failed to reconnect to Postgres database: %s\n", datajoint_core_get_last_error_message());
        connection_free(my_conn);
        return 1;
    }

    // Read query results with cursor.
    if (connection_fetch_query(my_conn, "select * from students;", NULL, &cursor) != ErrorCode_Success)
    {
        printf("Failed to create cursor for query: %s\n", datajoint_core_get_last_error_message());
        connection_free(my_conn);
        return 1;
    }
    generically_print_cursor_results_with_buffer(cursor);
    cursor_free(cursor);
    cursor = NULL;

    connection_disconnect(my_conn);
    connection_free(my_conn);
    my_conn = NULL;

    getchar();
    return 0;
}