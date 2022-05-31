import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

// User wants to verify the GraphQL query in request body before sending the request
RequestObject req = findTestObject('Country/CountryQuerySchema');
String graphQLSchema = 
'''
type Continent {
	code: ID!
	name: String!
	countries: [Country!]!
}

type Country {
	code: ID!
	name: String!
	native: String!
	phone: String!
	continent: Continent!
	capital: String
	currency: String
	languages: [Language!]!
	emoji: String!
	emojiU: String!
	states: [State!]!
}
type State {
	code: String
	name: String!
	country: Country!
}
type Language {
	code: ID!
	name: String
	native: String
	rtl: Boolean!
}
input StringQueryOperatorInput {
	eq: String
	ne: String
	in: [String]
	nin: [String]
	regex: String
	glob: String
}
input CountryFilterInput {
	code: StringQueryOperatorInput
	currency: StringQueryOperatorInput
	continent: StringQueryOperatorInput
}
input ContinentFilterInput {
	code: StringQueryOperatorInput
}
input LanguageFilterInput {
	code: StringQueryOperatorInput
}

type Query {
	continents(filter: ContinentFilterInput): [Continent!]!
	continent(code: ID!): Continent
	countries(filter: CountryFilterInput): [Country!]!
	country(code: ID!): Country
	languages(filter: LanguageFilterInput): [Language!]!
	language(code: ID!): Language
}
'''

if (WS.validateGraphQLBodyAgainstSchema(req, graphQLSchema)) {
	res = WS.sendRequest(req)
}


