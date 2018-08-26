package token

// Token represents the decoded form of the token in, strangely enough, Church notation. This is to
// emulate a proper enum type when Go doesn't have one.
//
// For those who don't know Church notation, a token is invoked as a function, and it calls the
// first function if it is a member token, and the second if it is a service token.
type Token func(func(uint), func(string))

func newMemberToken(id uint) Token {
	return func(onMember func(uint), onService func(string)) {
		onMember(id)
	}
}

func newServiceToken(name string) Token {
	return func(onMember func(uint), onService func(string)) {
		onService(name)
	}
}
