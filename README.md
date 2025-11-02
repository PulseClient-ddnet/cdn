# DDNet Tee Generator CDN

This project serves as a Content Delivery Network (CDN) for DDNet (DDrace Network) tee skins. It's designed to generate custom tee skins on demand, cache them for performance, and synchronize with official DDNet skin repositories.

## Description

The DDNet Tee Generator CDN provides an API to create and serve custom tee skins for the DDraceNetwork game. It allows users to specify skin parameters like name, body color, and feet color to generate unique tee appearances. The system leverages caching to serve frequently requested skins quickly and synchronizes with DDNet's official skin sources to maintain an up-to-date collection.

## Routes

The following API routes are available:

*   **`GET /skin`**
    *   **Description**: Generates and returns a custom tee skin based on the provided query parameters.
    *   **Query Parameters**:
        *   `name`: (String, **Required**) The name of the skin. Whitespaces are replaced with underscores.
        *   `body`: (u32, Optional) The DDNet color value for the tee's body.
        *   `feet`: (u32, Optional) The DDNet color value for the tee's feet.

*   **`GET /skin/store`**
    *   **Description**: Returns a JSON array of the names of all currently stored (downloaded and synchronized) skins.

*   **`GET /skin/cache`**
    *   **Description**: Returns a JSON array of the names of all currently cached (generated) skins.

*   **`GET /uvs`**
    *   **Description**: Serves a folder containing all UV (unwrap) images. This route acts as a mirror for the UV data.

*   **`GET /health`**
    *   **Description**: Performs a health check and returns a `204 No Content` response if the service is operational.

*   **`GET /doc`**
    *   **Description**: Serves the OpenAPI documentation for the API.

## Created Folders

The application creates and utilizes the following folders:

*   `static/`: Store generated scalar doc `doc.html`.
*   `.cache/`: Used for storing generated tee skins to prevent redundant computations. These cached skins have a TTL (Time To Live) of 15 minutes.
*   `.store/`: Contains the downloaded base skin images from DDNet sources and a `lock.json` file which tracks metadata about these stored skins.

## Examples of Requests

Here are examples of how to interact with the API:

*   **Generate a skin named "my_custom_tee" with specific body and feet colors:**
    ```
    GET /skin?name=my_custom_tee&body=322&feet=322
    ```

*   **Generate a skin named "another_tee" with only a name:**
    ```
    GET /skin?name=another_tee
    ```

*   **Get a list of all stored skins:**
    ```
    GET /skin/store
    ```

*   **Get a list of all cached skins:**
    ```
    GET /skin/cache
    ```

*   **Check the health of the service:**
    ```
    GET /health
    ```

*   **Access the API documentation:**
    ```
    GET /doc
    ```

*   **Access the UV images:**
    ```
    GET /uvs
    ```

## License
MIT License
