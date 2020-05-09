table! {
    account_holder (id) {
        id -> Int4,
        username -> Text,
        password_hash -> Text,
        first_name -> Text,
        last_name -> Text,
        access_token -> Text,
    }
}

table! {
    account_transactions (id) {
        id -> Text,
        amount -> Float4,
        utc_timestamp -> Int4,
        iso_currency -> Nullable<Text>,
        name -> Nullable<Text>,
        category -> Nullable<Text>,
        pending -> Int4,
        bank_account_id -> Text,
        pending_transaction_id -> Nullable<Text>,
        transfer_transaction_id -> Nullable<Text>,
    }
}

table! {
    bank_account (id) {
        id -> Text,
        balance_current -> Float4,
        balance_available -> Float4,
        account_name -> Text,
        account_type -> Text,
        iso_currency -> Text,
        account_holder_id -> Int4,
    }
}

table! {
    categories (id) {
        id -> Text,
        description -> Text,
    }
}

table! {
    color (id) {
        id -> Uuid,
        hexCode -> Varchar,
    }
}

table! {
    expense (id) {
        paused -> Int4,
        id -> Int4,
        frequency_type -> Text,
        frequency_date -> Nullable<Int4>,
        name -> Text,
        description -> Nullable<Text>,
        utc_timestamp_created -> Int4,
        price_bound_lower -> Float8,
        price_bound_upper -> Float8,
    }
}

table! {
    expense_transaction (expense_id, transaction_id) {
        expense_id -> Int4,
        transaction_id -> Text,
    }
}

table! {
    media (id) {
        id -> Uuid,
        title -> Varchar,
        altText -> Varchar,
        #[sql_name = "type"]
        type_ -> Media_type_enum,
        url -> Varchar,
    }
}

table! {
    product (id) {
        id -> Uuid,
        length -> Float8,
        width -> Float8,
        height -> Float8,
        name -> Varchar,
        description -> Varchar,
        links -> Nullable<Array<Text>>,
        returnsPolicy -> Nullable<Varchar>,
        termsAndConditions -> Nullable<Varchar>,
    }
}

table! {
    product_additional_images_media (productId, mediaId) {
        productId -> Uuid,
        mediaId -> Uuid,
    }
}

table! {
    product_colors_color (productId, colorId) {
        productId -> Uuid,
        colorId -> Uuid,
    }
}

table! {
    product_main_image_media (productId, mediaId) {
        productId -> Uuid,
        mediaId -> Uuid,
    }
}

table! {
    product_tags_tag (productId, tagId) {
        productId -> Uuid,
        tagId -> Uuid,
    }
}

table! {
    role (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    room (id) {
        id -> Uuid,
        floorPlan -> Jsonb,
        title -> Varchar,
        description -> Nullable<Varchar>,
        roomType -> Room_roomtype_enum,
        public -> Bool,
        length -> Float8,
        width -> Float8,
        height -> Float8,
        floorArea -> Float8,
        wallArea -> Float8,
        roomShape -> Room_roomshape_enum,
        totalShares -> Int4,
        parentRoomId -> Nullable<Uuid>,
        createdById -> Nullable<Uuid>,
    }
}

table! {
    room_additional_images_media (roomId, mediaId) {
        roomId -> Uuid,
        mediaId -> Uuid,
    }
}

table! {
    room_colors_color (roomId, colorId) {
        roomId -> Uuid,
        colorId -> Uuid,
    }
}

table! {
    room_contributors_user (roomId, userId) {
        roomId -> Uuid,
        userId -> Uuid,
    }
}

table! {
    room_main_image_media (roomId, mediaId) {
        roomId -> Uuid,
        mediaId -> Uuid,
    }
}

table! {
    room_products_product (roomId, productId) {
        roomId -> Uuid,
        productId -> Uuid,
    }
}

table! {
    room_tags_tag (roomId, tagId) {
        roomId -> Uuid,
        tagId -> Uuid,
    }
}

table! {
    tag (id) {
        id -> Uuid,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Tag_type_enum,
    }
}

table! {
    user (id) {
        id -> Uuid,
        firstName -> Varchar,
        lastName -> Varchar,
        dob -> Int4,
        bio -> Nullable<Varchar>,
        link -> Nullable<Varchar>,
        password -> Varchar,
        profileImage -> Nullable<Varchar>,
        email -> Varchar,
    }
}

table! {
    user_contributor_to_room (userId, roomId) {
        userId -> Uuid,
        roomId -> Uuid,
    }
}

table! {
    user_roles_role (userId, roleId) {
        userId -> Uuid,
        roleId -> Uuid,
    }
}

joinable!(account_transactions -> bank_account (bank_account_id));
joinable!(account_transactions -> categories (category));
joinable!(bank_account -> account_holder (account_holder_id));
joinable!(expense_transaction -> account_transactions (transaction_id));
joinable!(expense_transaction -> expense (expense_id));
joinable!(product_additional_images_media -> media (mediaId));
joinable!(product_additional_images_media -> product (productId));
joinable!(product_colors_color -> color (colorId));
joinable!(product_colors_color -> product (productId));
joinable!(product_main_image_media -> media (mediaId));
joinable!(product_main_image_media -> product (productId));
joinable!(product_tags_tag -> product (productId));
joinable!(product_tags_tag -> tag (tagId));
joinable!(room -> user (createdById));
joinable!(room_additional_images_media -> media (mediaId));
joinable!(room_additional_images_media -> room (roomId));
joinable!(room_colors_color -> color (colorId));
joinable!(room_colors_color -> room (roomId));
joinable!(room_contributors_user -> room (roomId));
joinable!(room_contributors_user -> user (userId));
joinable!(room_main_image_media -> media (mediaId));
joinable!(room_main_image_media -> room (roomId));
joinable!(room_products_product -> product (productId));
joinable!(room_products_product -> room (roomId));
joinable!(room_tags_tag -> room (roomId));
joinable!(room_tags_tag -> tag (tagId));
joinable!(user_contributor_to_room -> room (roomId));
joinable!(user_contributor_to_room -> user (userId));
joinable!(user_roles_role -> role (roleId));
joinable!(user_roles_role -> user (userId));

allow_tables_to_appear_in_same_query!(
    account_holder,
    account_transactions,
    bank_account,
    categories,
    color,
    expense,
    expense_transaction,
    media,
    product,
    product_additional_images_media,
    product_colors_color,
    product_main_image_media,
    product_tags_tag,
    role,
    room,
    room_additional_images_media,
    room_colors_color,
    room_contributors_user,
    room_main_image_media,
    room_products_product,
    room_tags_tag,
    tag,
    user,
    user_contributor_to_room,
    user_roles_role,
);
