# Streak API CLI Reference

Full command reference for `streak`.

## Commands

- [`streak api-keys`](#streak-api-keys)
- [`streak boxes`](#streak-boxes)
- [`streak comments`](#streak-comments)
- [`streak contacts`](#streak-contacts)
- [`streak meetings`](#streak-meetings)
- [`streak organizations`](#streak-organizations)
- [`streak pipeline-stages`](#streak-pipeline-stages)
- [`streak pipelines`](#streak-pipelines)
- [`streak search`](#streak-search)
- [`streak tasks`](#streak-tasks)
- [`streak teams`](#streak-teams)
- [`streak users`](#streak-users)

---

### `streak api-keys`

#### `streak api-keys create-api-key`

Creates an API key for the current user

`PUT /v1/apikeys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--team-key` | `string,null` | No |  |

#### `streak api-keys delete-api-key`

Deletes an API key by key

`DELETE /v1/apikeys/{apiKeyKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--api-key-key` | `string` | Yes |  |

#### `streak api-keys get-api-keys`

Lists API keys owned by the current user

`GET /v1/apikeys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer (int32)` | No | Number of items to return. |
| `--page` | `integer (int32)` | No | Zero-based page number |

---

### `streak boxes`

#### `streak boxes get-box-markdown`

Returns the box as one markdown document: an overview, its columns, contacts, organizations, linked boxes, tasks and pipeline, followed by its timeline. Every timestamp is ISO-8601 UTC. The document's structure is best-effort and may change; use the JSON endpoints for a stable contract.

`GET /v1/boxes/{boxKey}/md`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--box-key` | `string` | Yes |  |
| `--max-tokens` | `integer,null (int32)` | No | A token cap for the entire document, counted with OpenAI's o200k_base encoding. It covers the box sections first, then uses the remaining tokens for the timeline. A marker shows where entries were left out. If the box does not fit under the cap, a 400 error shows the minimum it needs. Omit the cap to return the full box. |

---

### `streak comments`

#### `streak comments create-comment`

Creates a comment on a box

`POST /v2/boxes/{boxKey}/comments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--box-key` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `streak comments delete-comment`

Deletes a comment by key

`DELETE /v2/comments/{commentKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--comment-key` | `string` | Yes |  |

#### `streak comments get-comment`

Returns a comment by key

`GET /v2/comments/{commentKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--comment-key` | `string` | Yes |  |

#### `streak comments get-comments`

Lists comments on a box

`GET /v2/boxes/{boxKey}/comments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--box-key` | `string` | Yes |  |

#### `streak comments react-to-comment`

Adds an emoji reaction to a comment

`POST /v2/comments/{commentKey}/react`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--comment-key` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `streak comments unreact-to-comment`

Removes an emoji reaction from a comment

`POST /v2/comments/{commentKey}/unreact`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--comment-key` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `streak comments update-comment`

Updates a comment by key

`POST /v2/comments/{commentKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--comment-key` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `streak contacts`

#### `streak contacts create-contact`

Creates a contact from its basic details. Links and custom fields require a subsequent update.

`POST /v2/teams/{teamKey}/contacts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--team-key` | `string` | Yes | Key of the team. |
| `--get-if-existing` | `boolean,null` | No | Return an existing email match instead of creating a contact. When getIfExisting=true, an email match is returned without merging new values. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `streak contacts delete-contact`

Deletes a contact and schedules cleanup of its links. A missing contact returns 404.

`DELETE /v2/contacts/{contactKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--contact-key` | `string` | Yes | Key of the contact. |

#### `streak contacts get-contact`

Returns an accessible contact by key.

`GET /v2/contacts/{contactKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--contact-key` | `string` | Yes | Key of the contact. |

#### `streak contacts get-contacts`

Returns accessible contacts indexed by their requested keys. Missing and inaccessible contacts are omitted.

`POST /v2/contacts/batch/get`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `streak contacts list-contacts`

Lists contacts in stable order. Pass the returned cursor to continue; keep the same after filter across pages. A full last page may be followed by an empty page.

`GET /v2/teams/{teamKey}/contacts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--team-key` | `string` | Yes | Key of the team. |
| `--after` | `integer (int64)` | No | Include contacts last saved at or after this epoch-seconds timestamp. |
| `--cursor` | `string` | No | Cursor returned by the previous response. Omit or leave blank to start from the beginning. |
| `--limit` | `integer (int32)` | No | Maximum number of results to return. Defaults to 100. Values above 1000 are capped. |

#### `streak contacts update-contact`

Applies supplied changes. Null or omitted fields remain unchanged.

`POST /v2/contacts/{contactKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--contact-key` | `string` | Yes | Key of the contact. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `streak meetings`

#### `streak meetings create-meeting`

Creates a meeting or call log on a box

`POST /v2/boxes/{boxKey}/meetings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--box-key` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `streak meetings delete-meeting`

Deletes a meeting or call log by key

`DELETE /v2/meetings/{meetingKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--meeting-key` | `string` | Yes |  |

#### `streak meetings get-meeting`

Returns a meeting or call log by key

`GET /v2/meetings/{meetingKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--meeting-key` | `string` | Yes |  |

#### `streak meetings get-meetings`

Lists meetings and call logs on a box

`GET /v2/boxes/{boxKey}/meetings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--box-key` | `string` | Yes |  |
| `--limit` | `integer (int32)` | No | Number of items to return. |
| `--page` | `integer (int32)` | No | Zero-based page number |

#### `streak meetings update-meeting`

Updates a meeting or call log by key

`POST /v2/meetings/{meetingKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--meeting-key` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `streak organizations`

#### `streak organizations create-organization`

Creates an organization in a team. Available enrichment data may fill missing details. Provide a name or at least one domain. Set custom fields and relationships in a subsequent update.

`POST /v2/teams/{teamKey}/organizations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--team-key` | `string` | Yes | Key of the team. |
| `--get-if-existing` | `boolean` | No | Return an existing organization matching a supplied domain instead of creating another organization. The supplied values are not merged into the existing organization. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `streak organizations delete-organization`

Deletes an organization and schedules removal of its relationships and box links.

`DELETE /v2/organizations/{organizationKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--organization-key` | `string` | Yes | Key of the organization. |

#### `streak organizations get-organization`

Returns an organization accessible to the current user, including custom fields and relationships.

`GET /v2/organizations/{organizationKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--organization-key` | `string` | Yes | Key of the organization. |

#### `streak organizations get-organizations`

Returns organizations keyed by their organization keys. Missing and inaccessible organizations are omitted.

`POST /v2/organizations/batch/get`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `streak organizations list-organizations`

Returns organizations in a team, ordered by key. Pass the returned cursor to retrieve the next page.

`GET /v2/teams/{teamKey}/organizations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--team-key` | `string` | Yes | Key of the team. |
| `--cursor` | `string` | No | Cursor returned by the previous response. Omit or leave blank to start from the beginning. |
| `--limit` | `integer (int32)` | No | Maximum number of results to return. Defaults to 100. Values above 1000 are capped. |

#### `streak organizations update-organization`

Updates an organization. Omitted and null properties are unchanged. Supplied lists replace existing lists, subject to field validation.

`POST /v2/organizations/{organizationKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--organization-key` | `string` | Yes | Key of the organization. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `streak pipeline-stages`

#### `streak pipeline-stages create-stage`

Creates a stage at the end of the pipeline's stage order.

`POST /v2/pipelines/{pipelineKey}/stages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pipeline-key` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `streak pipeline-stages delete-stage`

Deletes an empty stage. The final stage in a pipeline cannot be deleted.

`DELETE /v2/pipelines/{pipelineKey}/stages/{stageKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pipeline-key` | `string` | Yes |  |
| `--stage-key` | `string` | Yes |  |

#### `streak pipeline-stages get-stage`

Returns a stage in a pipeline.

`GET /v2/pipelines/{pipelineKey}/stages/{stageKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pipeline-key` | `string` | Yes |  |
| `--stage-key` | `string` | Yes |  |

#### `streak pipeline-stages list-stages`

Returns the pipeline's stages keyed by stage key.

`GET /v2/pipelines/{pipelineKey}/stages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pipeline-key` | `string` | Yes |  |

#### `streak pipeline-stages update-stage`

Updates the supplied stage name or colors.

`POST /v2/pipelines/{pipelineKey}/stages/{stageKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pipeline-key` | `string` | Yes |  |
| `--stage-key` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `streak pipelines`

#### `streak pipelines create-field`

Creates a custom field in a pipeline.

`POST /v2/pipelines/{pipelineKey}/fields`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pipeline-key` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `streak pipelines create-pipeline`

Creates a basic pipeline.

`POST /v2/pipelines`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `streak pipelines delete-field`

Deletes a custom field and clears the value from all the pipeline's boxes.

`DELETE /v1/pipelines/{pipelineKey}/fields/{fieldKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--field-key` | `string` | Yes |  |
| `--pipeline-key` | `string` | Yes |  |

#### `streak pipelines delete-pipeline`

Deletes an empty pipeline.

`DELETE /v2/pipelines/{pipelineKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pipeline-key` | `string` | Yes |  |

#### `streak pipelines get-field`

Returns a field by its key within a pipeline.

`GET /v1/pipelines/{pipelineKey}/fields/{fieldKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--field-key` | `string` | Yes |  |
| `--pipeline-key` | `string` | Yes |  |

#### `streak pipelines get-pipeline`

Returns a pipeline by key.

`GET /v2/pipelines/{pipelineKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pipeline-key` | `string` | Yes |  |

#### `streak pipelines list-fields`

Returns the fields in a pipeline in their stored order.

`GET /v1/pipelines/{pipelineKey}/fields`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pipeline-key` | `string` | Yes |  |

#### `streak pipelines list-pipelines`

Lists pipelines accessible to the current user.

`GET /v2/pipelines`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--sort-by` | `string,null` | No |  |
| `--limit` | `integer (int32)` | No | Number of items to return. |
| `--page` | `integer (int32)` | No | Zero-based page number |

#### `streak pipelines update-field`

Updates a field's name, default value, options, or AI settings.

`POST /v1/pipelines/{pipelineKey}/fields/{fieldKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--field-key` | `string` | Yes |  |
| `--pipeline-key` | `string` | Yes |  |
| `--add-only` | `boolean` | No | Append dropdown or tag options while retaining existing options. Defaults to false. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `streak pipelines update-pipeline`

Updates the supplied Pipeline fields. Omitted fields remain unchanged.

`POST /v2/pipelines/{pipelineKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pipeline-key` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `streak search`

#### `streak search search`

Searches visible boxes, contacts, and organizations by query, or visible boxes by exact name. Exactly one of query or name must be provided.

`GET /v1/search`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name` | `string,null` | No | Exact box name to search for. |
| `--page` | `integer (int32)` | No | Zero-based page number. |
| `--pipeline-key` | `array,null` | No | Pipeline keys to constrain the search to. |
| `--query` | `string,null` | No | Full-text query to search across boxes, contacts, and organizations. |
| `--stage-key` | `array,null` | No | Stage keys to constrain the search to. |
| `--team-key` | `array,null` | No | Team keys to constrain the search to. |

---

### `streak tasks`

#### `streak tasks create-task`

Creates a task on a box.

`POST /v2/boxes/{boxKey}/tasks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--box-key` | `string` | Yes | Key for the box to create the task on. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `streak tasks delete-task`

Deletes a task by key.

`DELETE /v2/tasks/{taskKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--task-key` | `string` | Yes | Key for the task to delete. |

#### `streak tasks get-assigned-tasks`

Lists tasks across all pipelines and boxes. By default, returns incomplete tasks assigned to the current user before tomorrow in the user's timezone, which includes today's and overdue tasks. Use userKeys, pipelineKey, includeCompleted, direction, limit, and sortOrder to page through other agenda slices.

`GET /v2/tasks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--direction` | `string,null` | No | Agenda direction. Accepted values are desc, past, asc, and future. Defaults to desc. |
| `--include-completed` | `boolean` | No | Whether completed tasks should be included in the returned agenda. |
| `--limit` | `integer (int32)` | No | Maximum number of tasks to return per assigned user. |
| `--pipeline-key` | `string,null` | No | Key for the pipeline to filter tasks to. Omit to search across all visible pipelines. |
| `--sort-order` | `string,null` | No | Opaque 30-digit sort cursor returned as task.sortOrder. Omit to use tomorrow at local midnight as the cursor boundary. |
| `--user-keys` | `array,null` | No | User keys whose assigned tasks should be returned. Omit to return tasks assigned to the current user. |

#### `streak tasks get-task`

Returns a single task by key.

`GET /v2/tasks/{taskKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--task-key` | `string` | Yes | Key for the task to return. |

#### `streak tasks get-tasks-for-box`

Lists tasks on a box, ordered by creation date.

`GET /v2/boxes/{boxKey}/tasks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--box-key` | `string` | Yes | Key for the box whose tasks should be listed. |
| `--limit` | `integer (int32)` | No | Number of items to return. Maximum: 1000 |
| `--page` | `integer (int32)` | No | Zero-based page number |

#### `streak tasks update-task`

Updates a task by key. Omitted body fields are left unchanged.

`POST /v2/tasks/{taskKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--task-key` | `string` | Yes | Key for the task to update. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `streak teams`

#### `streak teams create-team`

Creates a team with an initial member roster.

`POST /v2/teams`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `streak teams get-current-user-teams`

Lists the teams the current user belongs to, newest first.

`GET /v2/users/me/teams`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer (int32)` | No | Number of items to return. |
| `--page` | `integer (int32)` | No | Zero-based page number |

#### `streak teams get-team`

Returns a team by key.

`GET /v2/teams/{teamKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--team-key` | `string` | Yes | Key for the team to return. |

#### `streak teams update-team`

Updates a team by key. Omitted body fields are left unchanged.

`POST /v2/teams/{teamKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--team-key` | `string` | Yes | Key for the team to update. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `streak users`

#### `streak users get-current-user`

Returns information about the currently authenticated user

`GET /v1/users/me`

#### `streak users get-user`

Returns information about a specific user by their key

`GET /v1/users/{userKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--user-key` | `string` | Yes |  |

#### `streak users get-users-on-team`

Returns all the users on a single team including archived users, payer only users, agents, owners and regular members.

`GET /v2/teams/{teamKey}/users`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--team-key` | `string` | Yes |  |

#### `streak users update-user`

Updates a specific user by their key

`POST /v1/users/{userKey}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--user-key` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

## Global flags

These flags are available on every command:

| Flag | Description |
|------|-------------|
| `--dry-run` | Print the HTTP request without sending it |
| `--json <JSON\|->` | Supply the request body as JSON (or `-` for stdin) |
| `--params <JSON>` | Merge extra parameters as JSON |
| `--format <json\|table\|yaml\|csv>` | Output format (default: `json`) |
| `--output <PATH>` | Write binary responses to a file |
| `--base-url <URL>` | Override the API base URL |
| `--no-extract` | Print the full response body instead of the `x-fern-sdk-return-value` extraction |
| `--no-retry` | Disable retries declared by `x-fern-retries`, including network errors |
| `-q, --quiet` | Suppress stdout on success |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

Operations the spec describes how to page (via `x-fern-pagination` or a root `page_token` parameter) also accept:

| Flag | Description |
|------|-------------|
| `--page-all` | Auto-paginate and stream all results |
| `--page-limit <N>` | Max pages to fetch (default: `10`) |
| `--page-delay <MS>` | Delay between page fetches in milliseconds (default: `100`) |
| `--no-pager` | Disable the pager even on interactive terminals |

Operations the spec marks as streaming (via `x-fern-streaming`) also accept:

| Flag | Description |
|------|-------------|
| `--no-stream` | Buffer the streaming response and print it as a single value once complete |
