import { expect, test } from '@playwright/test';

const cityResult = {
  kind: 'city',
  label: 'Toronto, ON',
  cityId: 1,
  cityName: 'Toronto',
  citySlug: 'toronto',
  provinceCode: 'ON',
  areaId: null,
  areaName: null,
  areaSlug: null,
  latitude: 43.6532,
  longitude: -79.3832,
};

const initialListing = {
  id: 101,
  slug: 'amina-patel',
  fullName: 'Dr. Amina Patel',
  credentials: 'MD',
  phone: '416-555-0134',
  email: 'frontdesk@harbourfamily.example',
  profileUrl: null,
  clinic: {
    id: 12,
    name: 'Harbour Family Clinic',
    slug: 'harbour-family-clinic',
    addressLine1: '120 Queens Quay W',
    addressLine2: null,
    municipality: 'Toronto',
    provinceCode: 'ON',
    postalCode: 'M5J 2N8',
    phone: '416-555-0134',
    fax: null,
    email: 'frontdesk@harbourfamily.example',
    websiteUrl: null,
    latitude: 43.6408,
    longitude: -79.3818,
  },
  area: {
    id: 8,
    name: 'Downtown Toronto',
    slug: 'downtown-toronto',
  },
  status: {
    family_doctor_id: 101,
    current_status: 'unknown',
    last_reported_at: null,
    last_confirmed_accepting_at: null,
    last_confirmed_accepting_days_ago: null,
  },
};

test('searches a city, shows results, and confirms accepting status', async ({
  page,
}) => {
  let confirmRequestCount = 0;

  await page.route('**/api/cities/search?**', async (route) => {
    const url = new URL(route.request().url());

    expect(url.searchParams.get('q')).toContain('Tor');

    await route.fulfill({
      contentType: 'application/json',
      body: JSON.stringify({ results: [cityResult] }),
    });
  });

  await page.route('**/api/cities/toronto/doctors?**', async (route) => {
    await route.fulfill({
      contentType: 'application/json',
      body: JSON.stringify({
        city: {
          id: 1,
          name: 'Toronto',
          slug: 'toronto',
          provinceCode: 'ON',
          latitude: 43.6532,
          longitude: -79.3832,
        },
        listings: [initialListing],
      }),
    });
  });

  await page.route(
    '**/api/doctors/101/confirm-accepting',
    async (route) => {
      confirmRequestCount += 1;

      expect(route.request().method()).toBe('POST');
      await new Promise((resolve) => setTimeout(resolve, 250));

      await route.fulfill({
        contentType: 'application/json',
        body: JSON.stringify({
          doctorId: 101,
          report: {
            id: 901,
            reportKind: 'confirm_accepting',
            reportedStatus: 'accepting',
            note: null,
            submittedAt: '2026-07-21T00:00:00.000Z',
          },
          status: {
            family_doctor_id: 101,
            current_status: 'accepting',
            last_reported_at: '2026-07-21T00:00:00.000Z',
            last_confirmed_accepting_at: '2026-07-21T00:00:00.000Z',
            last_confirmed_accepting_days_ago: 0,
          },
        }),
      });
    },
  );

  await page.goto('/');

  await page.getByLabel('Search by city or area').fill('Tor');
  await page.getByRole('option', { name: /Toronto, ON/ }).click();

  await expect(page).toHaveURL(/\/results\?city=toronto/);
  await expect(
    page.getByRole('heading', { level: 1, name: 'Toronto, ON' }),
  ).toBeVisible();
  await expect(page.getByText('Dr. Amina Patel')).toBeVisible();
  await expect(page.getByText('Harbour Family Clinic')).toBeVisible();
  await expect(page.getByText('No confirmation')).toBeVisible();

  await page.getByRole('button', { name: 'Still accepting' }).click();

  await expect(page.getByText('Updating recency now...')).toBeVisible();
  await expect(page.getByText('Confirmed today')).toBeVisible();
  await expect(page.getByText('Confirmation recorded.')).toBeVisible();
  expect(confirmRequestCount).toBe(1);
});
