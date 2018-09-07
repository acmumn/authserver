table! {
    identity_jwt_escrow (id) {
        id -> Unsigned<Integer>,
        member_id -> Unsigned<Integer>,
        uuid -> Binary,
        created -> Datetime,
    }
}

table! {
    members_members (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        studentId -> Char,
        x500 -> Nullable<Varchar>,
        card -> Nullable<Char>,
        email -> Varchar,
    }
}

joinable!(identity_jwt_escrow -> members_members (member_id));

allow_tables_to_appear_in_same_query!(
    identity_jwt_escrow,
    members_members,
);
